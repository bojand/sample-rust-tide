# sample-rust-tide

Sample [https://github.com/http-rs/tide](Tide) application for DigitalOcean App Platform.

[![Deploy to DO](https://www.deploytodo.com/do-btn-blue.svg)](https://cloud.digitalocean.com/apps/new?repo=https://github.com/bojand/sample-rust-tide/tree/main)

Uses [https://crates.io/crates/cargo-build-deps](cargo-build-deps) crate to help speed up builds by building the dependences in a separate layer that's cached and reused on subsequent builds when only the application source code changes. 

Happy hacking!