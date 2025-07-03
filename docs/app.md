# GTS-RS implementation details

GTS-RS is a Rust application that allows sending and receiving Pokémon from Gen IV and V Pokémon games using the GTS system.

It consists of the following parts:
 * A DNS server that redirects GTS requests to this application.
 * A HTTP server that serves the GTS requests.
 * A collection of utilities to handle Pokémon and GTS data.

The HTTP server component of the application is developed using [Actix Web](https://actix.rs/).

The DNS server component of the application uses [Hickory DNS](https://hickory-dns.org/) (more specifically, [hickory-client](https://docs.rs/hickory-client/latest/hickory_client/index.html)) to redirect requests to a real DNS server, and potentially modify the responses.

The utilities for handling Pokémon and GTS data are implemented as a small library, [`pkm_utils`](../src/pkm_utils/).
