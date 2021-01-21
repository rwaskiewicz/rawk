# r-awk

A subset of [awk](https://en.wikipedia.org/wiki/AWK) written in [Rust](https://www.rust-lang.org/).

## Running
The project is currently compiled using `rustc 1.49.0 (e1884a8e3 2020-12-29)`

At this time, only running the REPL is supported as the project is in active development.
```commandline
RUST_LOG=debug cargo run
r-awk - a subset of awk written in Rust
r-awk >
```

### Logging
The `env_logger` crate is used as the implementation behind the `log` facade. Instructions for configuring log levels
can be found in the crate's [documentation](https://docs.rs/env_logger/0.8.2/env_logger/).

By default, the REPL will run with the ['Info' log filter](https://docs.rs/env_logger/0.8.2/env_logger/struct.Builder.html).

## References
- [IEEE Std 1003.1-2017](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/awk.html) - the awk specification
