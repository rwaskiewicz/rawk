```console
$ rawk
rawk 0.1.0

```

```console
$ rawk --help
awk, implemented in Rust

Usage: rawk [OPTIONS] [program]

Arguments:
  [program]  

Options:
  -f, --file <file>         Runs an awk program
  -q, --quick               Runs a single line of awk code without data, then terminates
  -k, --eval                Runs a single line of awk code, then terminates
  -F <field_separator>      Sets the field separator character/regex for parsing data [default: " "]
  -h, --help                Print help information
  -V, --version             Print version information

```

```console
$ rawk -V
rawk 0.1.0

```

```console
$ rawk --version
rawk 0.1.0

```

```console
$ {
> foo=3;
> bar+=foo;
> print bar;
> }
3

```