//! A simple Rocket application, based on the example in [Getting Started][].
//!
//! [Getting Started]: https://rocket.rs/v0.4/guide/getting-started/

#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{self, get, routes};

/// Declare a handler.
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Start our server.
fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
