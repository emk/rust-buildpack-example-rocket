# Example application: Rust, Rocket & Heroku

[![Build Status](https://travis-ci.org/emk/rust-buildpack-example-rocket.svg?branch=master)](https://travis-ci.org/emk/rust-buildpack-example-rocket)

[Rust][] is a language suitable for high-peformance, reliable web services. [Rocket][] is a rich, user-friendly web framework. [Heroku][] deploys, manages and scales web applications.

[Rust]: https://www.rust-lang.org/
[Rocket]: https://rocket.rs/
[Heroku]: https://www.heroku.com/

To deploy this application to Heroku, use this button:

[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy)

Or, if you'd prefer to use the command line, try running:

``` sh
git clone https://github.com/emk/rust-buildpack-example-rocket.git
cd rust-buildpack-example-rocket

# Install a known-good nightly build.
rustup install nightly-2018-12-08
rustup override set nightly-2018-12-08

heroku create --buildpack emk/rust
git push heroku master
```

This should make a local copy of this application and deploy it to Heroku.

For further instructions, see the [page for this buildpack][buildpack].

[buildpack]: https://github.com/emk/heroku-buildpack-rust

## Rocket and nightly Rust

This application works with a specific version of nightly Rust, specified in [`./rust-toolchain`](./rust-toolchain). The Rocket framework currently requires some nightly-only features, although this is expected to change in the future Rocket 0.5.x series.
