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
mod dns_server;
mod http_server;

use crate::{dns_server::DNSServer, http_server::run_http_server};
use futures::future::join;
use is_superuser::is_superuser;
use std::io::{Error, ErrorKind, Result};

fn print_license() {
    println!(
        "
        GTS-RS  Copyright (C) 2025  Bolu <bolu@tuta.io>
        This program comes with ABSOLUTELY NO WARRANTY.
        This is free software, and you are welcome to redistribute it
        under certain conditions.
        "
    );
}

// Log level: default to "info" for release builds, and "debug" for debug builds.
#[cfg(debug_assertions)]
const DEFAULT_LOG_LEVEL: &str = "debug";
#[cfg(not(debug_assertions))]
const DEFAULT_LOG_LEVEL: &str = "info";

#[tokio::main]
async fn main() -> Result<()> {
    print_license();

    // Check for superuser privileges:
    if !is_superuser() {
        eprintln!("This program must be run as superuser.");
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            "Not running as superuser",
        ));
    }

    // Initialize the logger; with the default level for this build.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(DEFAULT_LOG_LEVEL))
        .init();

    // Create and run servers, print exteral IP:
    let dns_server = DNSServer::new(None)
        .await
        .expect("Could not create the DNS server");

    let ip = dns_server.ip();
    println!("GTS-RS servers running on IP: {}", ip);

    let dns_handle = tokio::spawn(async move {
        dns_server
            .run()
            .await
            .expect("The DNS server failed to run");
    });

    let http_handle = run_http_server().expect("The HTTP server failed to run.");

    // Await for both servers to finish (which should never happen):
    let (http_result, dns_result) = join(http_handle, dns_handle).await;
    http_result.expect("The HTTP server failed to run");
    dns_result.expect("The DNS server failed to run");

    // The app will actually terminate when it is killed, e.g., with Ctrl+C.

    Ok(())
}
