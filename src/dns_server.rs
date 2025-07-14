/*
 * GTS-RS - Rust tool for downloading/uploading Pokémon to Gen IV/V games via the in-game GTS.
 * (Rust re-implementation of IR-GTS-MG: https://github.com/ScottehMax/IR-GTS-MG/tree/gen-5)
 * Copyright (C) 2025  Bolu <bolu@tuta.io>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use futures::StreamExt;
use hickory_client::{
    client::Client as DNSClient,
    proto::{
        op::message::Message,
        rr::{rdata::A, record_data::RData, RecordType},
        runtime::TokioRuntimeProvider,
        udp::UdpClientStream,
        xfer::{DnsHandle, DnsResponse},
    },
};
use std::{
    fmt,
    io::{Error, ErrorKind, Result},
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    str::FromStr,
};

/// Port to listen to DNS requests on.
const LISTENING_PORT: u16 = 53;
/// Wildcard IP address to listen to all IPv4 interfaces on this system.
const ALL_V4_INTERFACES: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
/// Wildcard port to make the OS assign an arbitrary port automatically.
const ANY_PORT: u16 = 0;

/// A DNS server that proxies requests to a real DNS server, and modifes certain responses to
/// impersonate Pokémon's GTS servers.
pub struct DNSServer {
    real_dns: DNSClient,
    real_dns_ip: Ipv4Addr, // Stored only to display on print.
    proxy_ip: Ipv4Addr,
    listening_socket: UdpSocket,
}

/// Implements the Display trait for DNSServer to provide a string representation for printing.
impl fmt::Display for DNSServer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DNSServer(IP {}, listening on {}, proxying DNS {}:53)",
            self.proxy_ip,
            self.listening_socket
                .local_addr()
                .expect("Could not get the local IP of the listeing socket"),
            self.real_dns_ip
        )
    }
}

/// Implements the Debug trait for DNSServer to provide a debug string representation.
#[cfg(debug_assertions)]
impl fmt::Debug for DNSServer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DNSServer {{\n    real_dns_ip: {:#?},\n    proxy_ip: {:#?},\n    listening_socket: {:#?}\n}}",
            self.real_dns_ip, self.proxy_ip, self.listening_socket
        )
    }
}

impl DNSServer {
    /// Creates a new instance of the DNSServer.
    ///
    /// # Arguments
    /// * `ip_to_proxy` - \[Optional\] The IP address of the real DNS server to proxy requests to. If
    ///   `None`, it defaults to `178.62.43.212`.
    pub async fn new(ip_to_proxy: Option<String>) -> Result<Self> {
        // Unpack the IP of the real DNS server to query:
        let ip_to_proxy = match ip_to_proxy {
            Some(ip) => ip,
            None => "178.62.43.212".to_string(), // Default DNS server.
        };
        let ip_to_proxy = Ipv4Addr::from_str(&ip_to_proxy)
            .expect(format!("[DNS resolver] Invalid IP address: {}", ip_to_proxy).as_str());
        let addr_to_proxy = SocketAddr::new(ip_to_proxy.into(), 53);

        // Create a DNS client to query the real DNS server (in a background thread):
        let udp_connection =
            UdpClientStream::builder(addr_to_proxy, TokioRuntimeProvider::default()).build();
        let (client, bg) = DNSClient::connect(udp_connection).await?;
        tokio::spawn(bg);

        // Get the local IP address for external connections of this DNS proxy:
        let proxy_ip = Self::get_proxy_ip(ip_to_proxy).await?;

        // Create and start the socket for the DNS connection with the client:
        let listening_socket = UdpSocket::bind((ALL_V4_INTERFACES, LISTENING_PORT))?;

        Ok(Self {
            real_dns: client,
            real_dns_ip: ip_to_proxy,
            proxy_ip,
            listening_socket,
        })
    }

    /// Gets the local IP for external connections of this DNS server.
    ///
    /// # Arguments
    /// * `remote_dns_ip` - The IP address of a server to make a dummy connection to.
    async fn get_proxy_ip(remote_dns_ip: Ipv4Addr) -> Result<Ipv4Addr> {
        // Create a dummy UDP socket, with no specific address:port, and make a dummy connection
        // with it. We can use the dummy connection to know the app's external IP address in the
        // local network.
        let socket = UdpSocket::bind((ALL_V4_INTERFACES, ANY_PORT))?; // Dummy socket.
        socket.connect((remote_dns_ip, 53u16))?; // Dummy connection.
        let local_addr = socket.local_addr()?;

        // Return the external IPv4 address:
        match local_addr.ip() {
            IpAddr::V4(ip) => Ok(ip),
            // An IPv6 should never occur, as an IPv4 address was requested above.
            IpAddr::V6(_) => Err(Error::new(
                ErrorKind::Other,
                "OS assigned an IPv6 address to the proxy, but only IPv4 is supported",
            )),
        }
    }

    /// Runs the DNS server, listening for requests.
    ///
    /// The DNS server listens for DNS requests, proxies them to a real DNS server, and modifies
    /// those that match Pokémon's GTS servers, changing their IP address to the proxy's IP.
    ///
    /// On proper execution (i.e. no errors), this function does not return.
    pub async fn run(&self) -> Result<()> {
        log::info!("DNS Proxy server started: {}.", self);

        let mut listening_buf = [0u8; 512];

        // Main loop for processing DNS requests:
        loop {
            // Wait for a DNS request from a client:
            let (_, client_address) = self.listening_socket.recv_from(&mut listening_buf)?;
            log::debug!("New DNS request received from {}", client_address);

            // Parse the received buffer into a DNS message and log the query:
            let Ok(dns_msg) = Message::from_vec(&listening_buf) else {
                log::debug!("Received message is not a DNS message.");
                continue;
            };
            let Some(dns_query) = dns_msg.query() else {
                log::debug!("Received DNS request with no query.");
                continue;
            };
            log::debug!("DNS Query: {}", dns_query);

            // Get the DNS message ID for the client, required to send the respone:
            let client_id = dns_msg.id();

            // Send the DNS request to the real server:
            'retry_dns_sending: loop {
                let mut exchange = self.real_dns.send(dns_msg.clone());
                // Handle all received responses:
                while let Some(response) = exchange.next().await {
                    // Retry sending the whole query if there was an error with the DNS query redirection:
                    if response.is_err() {
                        log::warn!(
                            "Error when querying the real DNS server ({}). Retrying...",
                            response
                                .err()
                                .expect("Error message missing for response matched as error")
                        );
                        continue 'retry_dns_sending;
                    }

                    // Modify respone to impersonate Pokémon's servers, and send back:
                    let modified_response = self.modify_response(response?, client_id);
                    self.listening_socket
                        .send_to(&modified_response.to_vec()?, client_address)?;
                }

                break 'retry_dns_sending;
            }
        }
    }

    /// Modifies the DNS responses from the real DNS server, to send back to the client.
    ///
    /// This function does two things:
    /// 1. Sets the ID of the response to match the ID of the client's request, so that the client
    ///    can accept the response.
    /// 2. Checks the response for any A records that match Nintendo's GTS servers, and modifies
    ///    the IP address of those records to the proxy's IP address.
    ///
    /// # Arguments
    /// * `response` - The DNS response received from the real DNS server, to be modified.
    /// * `id` - The ID of the DNS request from the client, to set in the response.
    ///
    /// # Returns
    /// * A modified `DnsResponse` with the ID set set to match the client's, and Nintendo's GTS
    ///   servers' IPs changed to the proxy's IP.
    fn modify_response(&self, mut response: DnsResponse, id: u16) -> DnsResponse {
        // Set the ID of the response to match the one of the client's request:
        response.set_id(id);

        // Modify the response to change Nintendo's servers' IP to our IP:
        for answer in response.answers_mut().iter_mut() {
            if answer.record_type() == RecordType::A {
                log::debug!("DNS returns IP {} for {}", answer.data(), answer.name());

                // Check if the A record matches Pokémon's GTS servers, and modify it:
                if answer.name().to_string() == "gamestats2.gs.nintendowifi.net.".to_string() {
                    answer.set_data(RData::A(A(self.proxy_ip)));
                    log::debug!("Modified answer: {}", answer);
                }
            }
        }

        response
    }

    /// Get the external IP address of the DNS server.
    pub fn ip(&self) -> Ipv4Addr {
        self.proxy_ip
    }
}
