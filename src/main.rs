use clap::{App, Arg};
use env_logger::{Builder, Env};
use log::{error, LevelFilter};

fn main() {
    Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .filter_module("rustyline", LevelFilter::Error)
        .init();

    // https://www.gnu.org/software/gawk/manual/html_node/Options.html
    let matches = App::new("r-awk")
        .version("0.0.1")
        .about("awk, implemented in Rust")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(false)
                .help("Runs an awk file"),
        )
        .arg(
            Arg::with_name("program")
                .index(1) // note this is the first positional argument, not the first argument as a whole
                .default_value("print $0;"),
        )
        .arg(
            // TODO: Remove this when `BEGIN` is implemented. We could use -w, but this is quicker
            Arg::with_name("quick")
                .short("q")
                .long("quick")
                .takes_value(false)
                .required(false)
                .help("Runs a single line of awk code without data, then terminates"),
        )
        .arg(
            Arg::with_name("eval")
                .short("k") // '-e' is taken already...
                .long("eval")
                .takes_value(false)
                .required(false)
                .help("Runs a single line of awk code, then terminates"),
        )
        // TODO: Implement -F for overriding FS
        .get_matches();

    let program = matches
        .value_of("program")
        .expect("awk program default is unspecified");
    let file_name = matches.value_of("file");
    match file_name {
        None => {
            let is_eval = matches.is_present("eval");
            // TODO: Remove this when `BEGIN` is implemented
            let is_quick = matches.is_present("quick");
            rawk::run_prompt(program, &[], is_eval, is_quick);
        }
        Some(s) => {
            error!("TODO: Implement file parsing. Got file_name {}", s);
        }
    }
}
