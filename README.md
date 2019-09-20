# CSH Project Tracker

A webapp for suggesting and voting on CSH house projects.

## Getting Started

Make sure you've installed [Rust] and [Docker], then execute
the following:

[Rust]: https://rustup.rs
[Docker]: https://docs.docker.com/v17.09/engine/installation/

```
$ cargo install diesel_cli --no-default-features --features postgres
$ docker-compose -f deployments/development.yml up
$ diesel setup
$ diesel migration run
```

If you have any trouble installing the diesel_cli tool, head on over
to the [diesel getting started page] to find out more details. Most
likely you'll need to install the postgres client library on your
system.

[diesel getting started page]: http://diesel.rs/guides/getting-started/

Once you've got all of that installed, just run

```
cargo run
```
