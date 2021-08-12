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
            Arg::with_name("eval")
                .short("k") // '-e' is taken already...
                .long("eval")
                .takes_value(false)
                .required(false)
                .help("Runs a single line of awk code, then terminates"),
        )
        .get_matches();

    let file_name = matches.value_of("file");
    match file_name {
        None => {
            let is_eval = matches.is_present("eval");
            rawk::run_prompt(is_eval);
        }
        Some(s) => {
            error!("TODO: Implement file parsing. Got file_name {}", s);
        }
    }
}
