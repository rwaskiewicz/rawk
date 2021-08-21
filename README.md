# r-awk

A subset of [awk](https://en.wikipedia.org/wiki/AWK) written in [Rust](https://www.rust-lang.org/).

## Running
The project is currently compiled using `cargo 1.54.0 (5ae8d74b3 2021-06-22)`

At this time, only running the REPL is supported as the project is in active development.
```commandline
RUST_LOG=debug cargo run
r-awk > foo = 2;
r-awk > bar = 3;
r-awk > if (foo + bar > 5) { print foo + bar " is too high"; } else { baz = foo + bar; print baz " is not above the limit"; } 
[INFO  rawk::vm] 5 is not above the limit
r-awk > if (baz) print "baz is defined"; else print "baz is not defined";
[INFO  rawk::vm] baz is defined
```

### Logging
The `env_logger` crate is used as the implementation behind the `log` facade. Instructions for configuring log levels
can be found in the crate's [documentation](https://docs.rs/env_logger/0.8.2/env_logger/).

By default, the REPL will run with the ['Info' log filter](https://docs.rs/env_logger/0.8.2/env_logger/struct.Builder.html).

## References
- [IEEE Std 1003.1-2017](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/awk.html) - the POSIX specification for the awk language
