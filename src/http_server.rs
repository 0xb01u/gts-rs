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
use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{Server, ServiceRequest, ServiceResponse},
    error::Error as ActixError,
    get,
    http::StatusCode,
    middleware::{from_fn, Logger, Next},
    web::{scope, Query},
    App, HttpResponse, HttpResponseBuilder, HttpServer, Result as ActixResult,
};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use paste::paste;
use serde::Deserialize;
use sha1::{Digest, Sha1};
use std::{
    collections::HashMap,
    fs,
    io::{stdin, Result},
    net::Ipv4Addr,
    path::Path,
};

use pkm_utils::{
    gts::{GTSDeposit, GTSReception},
    pokemon::Pokemon,
};

/// Token used for some specific GTS response:
const GTS_TOKEN: &str = "c9KcX1Cry3QKS2Ai7yxL6QiQGeBGeQKR";
/// Salt used for generating the footer in Gen 5 responses.
const GEN5_SALT: &[u8; 20] = b"HZEdGCzcGGLvguqUEKQN";

/// Generates a proper response for the Gen 4 Pokémon games' GTS service, given the body of the
/// HTTP response.
///
/// This function mainly sets the appropriate headers and builds a response with it and the
/// provided body.
///
/// # Arguments
/// * `body` - The body of the HTTP response, containing the proper data to send to the client.
fn gts_response_gen4(body: impl MessageBody + 'static) -> HttpResponse<BoxBody> {
    // Response headers for the GTS service:
    // (These seem to be optional.)
    let headers = vec![
        ("Server", "Microsoft-IIS/6.0"),
        ("P3P", "CP='NOI ADMa OUR STP'"),
        ("cluster-server", "aphexweb3"),
        ("X-Server-Name", "AW4"),
        ("X-Powered-By", "ASP.NET"),
        ("Content-Type", "text/html"),
        (
            "Set-Cookie",
            "ASPSESSIONIDQCDBDDQS=JFDOAMPAGACBDMLNLFBCCNCI; path=/",
        ),
        ("Cache-control", "private"),
    ];

    // Build the response object, with the headers and the body, and try to return it:
    let mut response_builder = HttpResponseBuilder::new(StatusCode::OK);
    for header in headers {
        response_builder.append_header(header);
    }
    let response = response_builder.message_body(BoxBody::new(body));
    if let Ok(response) = response {
        response
    } else {
        log::error!("Failed to build response: {:?}", response.err());
        HttpResponse::InternalServerError().finish()
    }
}

/// Generates a proper response for the Gen 5 Pokémon games' GTS service, given the body of the
/// HTTP response.
///
/// This function mainly sets the appropriate headers, generates the Gen 5-exclusive footer, and
/// builds a response with them and the provided body.
///
/// The main difference with Gen 4 responses is that Gen 5 responses append a special footer to the
/// response's body, if it is not empty.
///
/// # Arguments
/// * `body` - The body of the HTTP response, containing the proper data to send to the client.
fn gts_response_gen5(body: impl MessageBody + 'static) -> HttpResponse<BoxBody> {
    // Check body size, and skip footer generation if empty:
    if body.size().is_eof() {
        return gts_response_gen4(body);
    }

    // Extract the body from the message in a format we can work with:
    let mut body_bytes = match body.try_into_bytes() {
        Ok(bytes) => bytes.to_vec(),
        Err(_) => {
            log::error!("Failed to convert body to bytes for base64 encoding");
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Generate and append the footer:
    let b64_body = URL_SAFE.encode(&body_bytes);
    let mut hasher = Sha1::new();
    hasher.update(GEN5_SALT);
    hasher.update(b64_body.as_bytes());
    hasher.update(GEN5_SALT);
    let footer = format!("{:x}", hasher.finalize());

    body_bytes.extend(footer.as_bytes());

    // Add the proper heads, similarly to Gen 4 responses:
    gts_response_gen4(body_bytes)
}

// Middleware functions to perform request pre-processing:

/// Macro to generate the middleware functions for Gen 4 and Gen 5.
///
/// As the middleware functions only differ in the GTS response generation function used,
/// a macro is used to avoid code repetition.
///
/// # Arguments
/// * `$gen` - The generation number for which to generate the middleware function (4 or 5).
macro_rules! handle_request {
    ($gen:literal) => {
        paste! {
            #[doc = concat!("Middleware function for handling requests in Gen ", stringify!($gen),
            " games.")]
            async fn [<handle_request_gen$gen>](
                req: ServiceRequest,
                next: Next<BoxBody>,
            ) -> ActixResult<ServiceResponse<BoxBody>, ActixError> {
                // Handle requests to unknown routes:
                if req.match_name().is_none() {
                    log::warn!("No route found for {}", req.path());

                    return Ok(req.into_response("").map_into_boxed_body());
                }
                // Handle token requets:
                // These include one query in the URL (i.e., they end with "?<key>=<value>").
                let args_map = Query::<HashMap<String, String>>::from_query(req.query_string())
                    .expect("Failed to parse URL args")
                    .into_inner();
                if args_map.len() == 1 {
                    return Ok(ServiceResponse::new(
                        req.request().clone(),
                        gts_response_gen4(GTS_TOKEN),
                    ));
                }

                // Progress request into the chain and build the proper response with the result:
                let (req, res) = next.call(req).await?.into_parts();
                let new_res = [<gts_response_gen$gen>](res.into_body());
                Ok(ServiceResponse::new(req, new_res))
            }
        }
    };
}

handle_request!(4);
handle_request!(5);

// GTS service endpoints:

// Gen 4 endpoints use the `/pokemondpds` root path.
//
// Gen 5 endpoints use the `/syachi2ds/web` root path.
//
// Most GTS services are serviced under the `/worldexchange` subpath, with the exception of
// `set_profile`. That is serviced under `/worldexchange` for Diamond, Pearl, Heargold and
// Soulsilver, but under the root path for Platinum and the Gen 5 games.
//
// Therefore, most functions service more than one endpoint. Commonly, they service 2 endpoints
// (one for Gen 4 and one for Gen 5). The "result" and "post" functions (`result_gen4` and
// `result_gen5`, and `post_gen4` and `post_gen5`) are the only ones that service only one endpoint
// each. Nevertheless, those function pairs are macro-generated to avoid code repetition.
//
// At the bottom of this file, in `run_http_server`, it can be seen how the different endpoints are
// serviced and how the web server map is constructed.

/// Macro to generate a default HTTP response, given only its body.
macro_rules! response_from_body {
    ($body:ident) => {
        HttpResponse::Ok().body($body)
    };
    ($body:expr) => {
        HttpResponse::Ok().body($body as &[u8])
    };
}

#[get("/common/setProfile.asp")]
async fn set_profile() -> HttpResponse {
    response_from_body!(&[0u8; 8])
}

#[get("/info.asp")]
async fn info() -> HttpResponse {
    log::info!("Connection established.");
    response_from_body!(b"\x01\x00")
}

#[derive(Deserialize)]
struct PostData {
    data: String,
}

/// Macro to generate the post endpoints for Gen 4 and Gen 5.
///
/// This macro is used to avoid code repetition, as the Gen 4 and Gen 5 post endpoints differ only
/// slightly: They have to indicate the internal GTS data structure that the received data belongs
/// to the corresponding generation.
///
/// # Arguments
/// * `$gen` - The generation number for which to generate the post endpoint function (4 or 5).
macro_rules! post_endpoint {
    ($gen:literal) => {
        paste! {
            #[get("/post.asp")]
            async fn [<post_gen$gen>](data: Query<PostData>) -> HttpResponse {
                log::info!("Receiving Gen {} Pokémon...", $gen);

                // Create the GTS deposit struct from the received base64 data:
                let deposit = match GTSDeposit::from_base64(&data.data, $gen == 5) {
                    Ok(deposit) => deposit,
                    Err(e) => {
                        log::error!("Failed to process Pokémon deposit: {}", e);
                        return response_from_body!(b"\x0c\x00");
                    }
                };

                // Extract the Pokémon and save it to disk:
                let pokemon = deposit.pokemon();
                let saved = pokemon
                    .save(None, None)
                    .expect(format!("Failed to save Gen {} Pokémon", $gen).as_str());
                if saved {
                    log::info!("Pokémon saved successfully.");
                } else {
                    log::warn!("Pokémon already saved. Skipping save.");
                }

                // Dump Pokémon to the debug output and a file:
                log::debug!("{:?}", pokemon);
                match fs::write(
                    "recv_pkm.log",
                    format!("{:?}", pokemon),
                ) {
                    Ok(_) =>{
                        if let Ok(file_metadata) = fs::metadata("recv_pkm.log") {
                            let mut file_permissions = file_metadata.permissions();
                            file_permissions.set_readonly(false);
                            if fs::set_permissions("recv_pkm.log", file_permissions).is_err() {
                                log::warn!("Failed to change permissions for `recv_pkm.log`");
                            } else {
                                log::info!("Pokémon data logged to `recv_pkm.log`.");
                            }
                        } else {
                            log::warn!("Failed to get metadata for `recv_pkm.log`");
                        }
                    }
                    Err(e) => log::error!("Failed to log received Pokémon to `recv_pkm.log`: {}", e)
                }

                response_from_body!(b"\x0c\x00")
            }
        }
    };
}

post_endpoint!(4);
post_endpoint!(5);

#[get("/search.asp")]
async fn search() -> HttpResponse {
    response_from_body!(b"")
}

/// Macro to generate the result endpoints for Gen 4 and Gen 5.
///
/// This macro is used to avoid code repetition, as the Gen 4 and Gen 5 result endpoints differ
/// only slightly: Both endpoints check which generation the Pokémon selected by the user belongs
/// to, and only allow those of the corresponding generation to be sent.
///
/// # Arguments
/// * `$gen` - The generation number for which to generate the result endpoint function (4 or 5).
#[rustfmt::skip] // Rust formatting messes up with this macro.
macro_rules! result_endpoint {
    ($gen:literal) => {
        paste! {
            #[get("/result.asp")]
            async fn [<result_gen$gen>]() -> HttpResponse {
                // Loop until a valid Pokémon is specified, or no Pokémon is sent:
                let pokemon = loop {
                    let mut path = String::new();

                    println!("Enter the path or drag the .pkm/.pk{} file here.", $gen);
                    println!("Leave blank to not send a Pokémon and proceed through the GTS \
                        (for deposits).");

                    // Read and sanitize the path, or skip:
                    if stdin().read_line(&mut path).is_err() {
                        log::error!("Error reading from stdin.");
                        continue;
                    } else if path.trim().is_empty() {
                        log::warn!("No Pokémon path provided; letting the game proceed to \
                            Pokémon deposit.");
                        return response_from_body!(b"\x05\x00");
                    }

                    path = path.trim().to_string();
                    if (path.starts_with("'") && path.ends_with("'"))
                        || path.starts_with("\"") && path.ends_with("\"")
                    {
                        path = path[1..path.len() - 1].to_string();
                    }

                    // Load the Pokémon struct and return it:
                    let pokemon_load = Pokemon::load(Path::new(&path));
                    let pokemon = match pokemon_load {
                        Ok(pokemon) => pokemon,
                        Err(e) => {
                            log::error!("Failed to load Gen {} Pokémon from {}: {}", $gen, path, e);
                            continue;
                        }
                    };
                    log::info!("Pokémon loaded from {} successfully.", path);

                    if pokemon.is_gen5() != ($gen == 5) {
                        log::error!("The Pokémon selected is not a Gen {} Pokémon.", $gen);
                        continue;
                    }

                    break pokemon;
                };

                // Build response:
                let body = GTSReception::from_pokemon(&pokemon).serialize();

                response_from_body!(body)
            }
        }
    };
}

result_endpoint!(4);
result_endpoint!(5);

#[get("/delete.asp")]
async fn delete() -> HttpResponse {
    response_from_body!(b"\x01\x00")
}

/// Wildcard IP address to listen to all IPv4 interfaces on this system.
const ALL_V4_INTERFACES: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
/// Port to listen to incoming HTTP requests on:
const LISTENING_PORT: u16 = 80;

/// Creates the HTTP server mimicking the Pokémon GTS service, starts it, and returns the server
/// instance.
///
/// The server is bound to port 80 (HTTP) on all IPv4 interfaces in the system.
pub fn run_http_server() -> Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            // Log actix HTTP server activity, if the log level is Debug or higher:
            .wrap(Logger::default().log_level(log::Level::Debug))
            // Endpoints/services:
            .service(
                scope("/pokemondpds")
                    .wrap(from_fn(handle_request_gen4))
                    .service(
                        scope("/worldexchange")
                            .service(info)
                            .service(post_gen4)
                            .service(search)
                            .service(result_gen4)
                            .service(delete),
                    )
                    .service(set_profile),
            )
            .service(
                scope("/syachi2ds/web")
                    .wrap(from_fn(handle_request_gen5))
                    .service(
                        scope("/worldexchange")
                            .service(info)
                            .service(post_gen5)
                            .service(search)
                            .service(result_gen5)
                            .service(delete),
                    )
                    .service(set_profile),
            )
    })
    // Disable signal handling, for exiting with Ctrl + C:
    .disable_signals()
    // Spawn just one worker, as (many) concurrent petitions are not expected:
    .workers(1)
    .bind((ALL_V4_INTERFACES, LISTENING_PORT))?;

    log::info!("Running HTTP server on {}", server.addrs()[0]);

    Ok(server.run())
}
