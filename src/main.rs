use clap::{App, Arg};
use env_logger::{Builder, Env};
use log::LevelFilter;
use rawk::runtime_config::RuntimeConfig;

fn main() {
    Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .filter_module("rustyline", LevelFilter::Error)
        .init();

    const PROGRAM_KEY: &str = "program";
    const QUICK_KEY: &str = "quick";
    const EVAL_KEY: &str = "eval";
    const FIELD_SEPARATOR_KEY: &str = "field_separator";
    const FILE_KEY: &str = "file";

    // https://www.gnu.org/software/gawk/manual/html_node/Options.html
    let matches = App::new("r-awk")
        .version("0.0.1")
        .about("awk, implemented in Rust")
        .arg(
            Arg::with_name(FILE_KEY)
                .short("f")
                .long(FILE_KEY)
                .takes_value(true)
                .required(false)
                .help("Runs an awk file"),
        )
        .arg(
            Arg::with_name(PROGRAM_KEY)
                .index(1) // note this is the first positional argument, not the first argument as a whole
                .default_value("print $0;"),
        )
        .arg(
            // TODO: Remove this when `BEGIN` is implemented. We could use -w, but this is quicker
            Arg::with_name(QUICK_KEY)
                .short("q")
                .long(QUICK_KEY)
                .takes_value(false)
                .required(false)
                .help("Runs a single line of awk code without data, then terminates"),
        )
        .arg(
            Arg::with_name(EVAL_KEY)
                .short("k") // '-e' is taken already...
                .long(EVAL_KEY)
                .takes_value(false)
                .required(false)
                .help("Runs a single line of awk code, then terminates"),
        )
        .arg(
            Arg::with_name(FIELD_SEPARATOR_KEY)
                .short("F")
                .takes_value(true)
                .required(false)
                .default_value(" ")
                .help("Sets the field separator character/regex for parsing data"),
        )
        .get_matches();

    let program = matches
        .value_of(PROGRAM_KEY)
        .expect("awk program default is unspecified");
    let field_separator = matches
        .value_of(FIELD_SEPARATOR_KEY)
        .map(|separator| separator.to_string())
        .expect("awk file separator default is unspecified");
    let file_name = matches.value_of(FILE_KEY).map(|name| name.to_string());

    let config: RuntimeConfig = RuntimeConfig::new(
        file_name,
        field_separator,
        matches.is_present(EVAL_KEY),
        matches.is_present(QUICK_KEY),
    );
    rawk::run_prompt(program, config);
}
