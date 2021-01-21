use clap::{App, Arg};
use env_logger::Builder;
use log::{error, LevelFilter};
use std::io;
use std::io::Write;

fn main() {
    let mut logger_builder = Builder::from_default_env();
    // TODO: This isn't overridable
    logger_builder.filter(None, LevelFilter::Info).init();

    let matches = App::new("r-awk")
        .version("0.0.1")
        .about("awk, implemented in Rust")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let file_name = matches.value_of("file");
    match file_name {
        None => {
            run_prompt();
        }
        Some(s) => {
            error!("TODO: Implement file parsing. Got file_name {}", s);
        }
    }
}

fn run_prompt() {
    println!("r-awk - a subset of awk written in Rust");

    let mut awk_line = String::new();

    print!("r-awk > ");
    io::stdout().flush().expect("Unable to flush STDOUT!");

    io::stdin()
        .read_line(&mut awk_line)
        .expect("failed to get r-awk line");
    print!("r-awk line to process: {}", awk_line);

    // When we had one, we would init a new VM on every loop. This won't be feasible long term, but
    // for now we can avoid the scary monsters under the bed with resetting state...
    rawk::startup_and_interpret_awk_line(awk_line);
}
