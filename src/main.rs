//! A simple Rocket application, based on the example in [Getting Started][].
//!
//! [Getting Started]: https://rocket.rs/v0.5-rc/guide/getting-started/
use rocket::{get, launch, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
