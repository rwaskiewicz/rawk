# r-awk

a toy [awk](https://en.wikipedia.org/wiki/AWK) written in [Rust](https://www.rust-lang.org/).

## Running
The minimum version required to run r-awk can be found under the `rust-version` key in the project's 
[`Cargo.toml`](./Cargo.toml) file.

At this time, only running a REPL is supported, as the project is in active development. Like a real awk, this r-awk
will take a single program from the STDIN when invoked. It then will prompt for input to serve as the data that is fed
into the program. The program below demonstrates running r-awk via `cargo run` and demonstrates the usage of 
[field variables](https://www.gnu.org/software/gawk/manual/gawk.html#Fields) with a comma (,) as a
[field separator](https://www.gnu.org/software/gawk/manual/html_node/Single-Character-Fields.html).

```commandline
cargo run -- -F, '{print $2 * $3 + $1;}'
1,2,3
7
4,5,6
34
```

At this time, only single line programs are supported in the REPL in the REPL are supported. Take 'fizzbuzz' for 
example:
```awk
{i=0; while (i < 100) { i=i+1; is_three_div = (i % 3 == 0); is_five_div = (i % 5 == 0); if (is_three_div && is_five_div) { print "fizzbuzz"; } else if (is_three_div) { print "fizz"; } else if (is_five_div) { print "buzz"; } else { print i; }}}
```
which, when split into multiple lines, is:
```awk
{
    i=0;
    while (i < 100) {
        i=i+1;
        is_three_div = (i % 3 == 0);
        is_five_div = (i % 5 == 0);
        if (is_three_div && is_five_div) {
            print "fizzbuzz";
        } else if (is_three_div) {
            print "fizz";
        } else if (is_five_div) {
            print "buzz";
        } else {
            print i;
        }
    }
}
```

Or with string concatenation: 
```awk
{i=0; while (i < 100) { result = ""; i=i+1; if (i % 3 == 0) { result = "fizz"; } if (i % 5 == 0) { result = result "buzz"; } if (!(i % 3 == 0) && !(i % 5 == 0)){ result=i; } print result; }}
```
```awk
{
    i=0; 
    while (i < 100) {
      result = "";
      i=i+1;
      if (i % 3 == 0) { 
        result = "fizz";
      } 
      if (i % 5 == 0) { 
        result = result "buzz";
      } 
      if (!(i % 3 == 0) && !(i % 5 == 0)){
        result=i;
      }
      print result;
    }
}
```

### Logging
The `env_logger` crate is used as the implementation behind the `log` facade. Instructions for configuring log levels
can be found in the crate's [documentation](https://docs.rs/env_logger/0.8.2/env_logger/).

By default, the REPL will run with the ['Info' log filter](https://docs.rs/env_logger/0.8.2/env_logger/struct.Builder.html).

Example usage:
```commandline
RUST_LOG=debug cargo run -- '{print $2 * $3 + $1;}'
```

## References
- [IEEE Std 1003.1-2017](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/awk.html) - the POSIX specification for the awk language
