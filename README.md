# r-awk

a toy [awk](https://en.wikipedia.org/wiki/AWK) written in [Rust](https://www.rust-lang.org/), feature incomplete.

## Minimum Rust Version

The minimum version required to compile and run r-awk can be found under the `rust-version` key in the project's [`Cargo.toml`](./Cargo.toml) file.

## Compiling

```commandline
cargo build --release
```

## Running

At this time, running a REPL is fully supported and has limited support for awk file reading. 
Like a real awk, this r-awk will take a single program from the STDIN when invoked.
It then will prompt for input to serve as the data that is fed into the program.

The program below demonstrates running a [compiled](#compiling) r-awk and demonstrates the usage of 
[field variables](https://www.gnu.org/software/gawk/manual/gawk.html#Fields) and
[field separators](https://www.gnu.org/software/gawk/manual/html_node/Single-Character-Fields.html).

```commandline
./rawk -F, '{print $2 * $3 + $1;}'
1,2,3
7
4,5,6
34
```

Patterns are supported:
```commandline
./rawk -F, '$1 > $2 {print "First is bigger";} $2 > $1 {print "Second is bigger";}'
1,2
Second is bigger
2,1
First is bigger
```

Multi-line programs are supported in the REPL.
Take 'fizzbuzz' for example:
```
./rawk '{
    for (i=0; i<=100; i=i+1) {
      result = "";
      if (i % 3 == 0) { 
        result = "fizz";
      }
      if (i % 5 == 0) {
        result = result "buzz";
      }
      if (result == "") {
        result = i;
      }
      print result;
    }
}'
```

Reading an awk program from a file:
```commandline
./rawk -- -f ./awk_examples/field_variables/it_prints_all_line_parts.awk
IN: alice 40 25
OUT: alice 40 25
```

### Logging
The `env_logger` crate is used as the implementation behind the `log` facade.
Instructions for configuring log levels can be found in the crate's [documentation](https://docs.rs/env_logger/0.8.2/env_logger/).

By default, the REPL will run with the ['Info' log filter](https://docs.rs/env_logger/0.8.2/env_logger/struct.Builder.html).

Example usage:
```commandline
RUST_LOG=debug cargo run -- '{print $2 * $3 + $1;}'
```

## References
- [IEEE Std 1003.1-2017](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/awk.html) - the POSIX specification for the awk language
