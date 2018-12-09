//! A simple Rocket application, based on the example in [Getting Started][].
//!
//! [Getting Started]: https://rocket.rs/v0.4/guide/getting-started/

#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{self, get, routes, Config};
use std::env;

/// Declare a handler.
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Configure Rocket to serve on the port requested by Heroku.
fn configure() -> Config {
    let mut config = Config::active().expect("could not load configuration");
    if let Ok(port_str) = env::var("PORT") {
        let port = port_str.parse().expect("could not parse PORT");
        config.set_port(port);
    }
    config
}

/// Start our server.
fn main() {
    rocket::custom(configure()).mount("/", routes![index]).launch();
}
