use clap::{Arg, ArgMatches, Command};
use env_logger::{Builder, Env};
use log::LevelFilter;
use rawk::runtime_config::RuntimeConfig;
use std::error::Error;
use std::fmt::Debug;
use std::fs;

#[derive(Debug)]
enum TempAwkReadFileError {
    FileDoesNotExist,
}

const PROGRAM_KEY: &str = "program";
const PROGRAM_FILE_KEY: &str = "file";

fn main() -> Result<(), Box<dyn Error>> {
    Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .filter_module("rustyline", LevelFilter::Error)
        .init();

    const VERSION_KEY: &str = "version";
    const QUICK_KEY: &str = "quick";
    const EVAL_KEY: &str = "eval";
    const FIELD_SEPARATOR_KEY: &str = "field_separator";

    // https://www.gnu.org/software/gawk/manual/html_node/Options.html
    let cmd_line_app = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("awk, implemented in Rust");

    let version_info = cmd_line_app.render_version();

    let cmd_line_matches = cmd_line_app
        .arg(
            Arg::new(VERSION_KEY)
                .short('V')
                .long(VERSION_KEY)
                .takes_value(false)
                .required(false)
                .help("Determine the current version of r-awk"),
        )
        .arg(
            Arg::new(PROGRAM_FILE_KEY)
                .short('f')
                .long(PROGRAM_FILE_KEY)
                .takes_value(true)
                .required(false)
                .multiple_occurrences(true)
                .number_of_values(1)
                .help("Runs an awk program"),
        )
        .arg(
            Arg::new(PROGRAM_KEY).index(1), // note this is the first positional argument, not the first argument as a whole
        )
        .arg(
            // TODO: Remove this when `BEGIN` is implemented. We could use -w, but this is quicker
            Arg::new(QUICK_KEY)
                .short('q')
                .long(QUICK_KEY)
                .takes_value(false)
                .required(false)
                .help("Runs a single line of awk code without data, then terminates"),
        )
        .arg(
            Arg::new(EVAL_KEY)
                .short('k') // '-e' is taken already...
                .long(EVAL_KEY)
                .takes_value(false)
                .required(false)
                .help("Runs a single line of awk code, then terminates"),
        )
        .arg(
            Arg::new(FIELD_SEPARATOR_KEY)
                .short('F')
                .takes_value(true)
                .required(false)
                .default_value(" ")
                .help("Sets the field separator character/regex for parsing data"),
        )
        .get_matches();

    if !cmd_line_matches.is_present(PROGRAM_KEY) && !cmd_line_matches.is_present(PROGRAM_FILE_KEY) {
        // clap handles version flags itself, use post-matching results to handle other cases where
        // we only wish to print the version info
        println!("{}", version_info.trim());
        return Ok(());
    }

    let program = get_awk_program(&cmd_line_matches);

    let field_separator = cmd_line_matches
        .value_of(FIELD_SEPARATOR_KEY)
        .map(|separator| separator.to_string())
        .expect("awk file separator default is unspecified");

    let config: RuntimeConfig = RuntimeConfig::new(
        None,
        field_separator,
        cmd_line_matches.is_present(EVAL_KEY),
        cmd_line_matches.is_present(QUICK_KEY),
    );
    rawk::run_program(&program, config);
    Ok(())
}

/// Retrieve an awk program from the command line
///
/// A program can be provided as a single argument from the command line, or through one or more
/// usages of the `-f progfile` flag, where `progfile` is the path to the awk program to run. if
/// more than one instance of `-f progfile` is provided, each `progfile` shall be read in the order
/// they are declared and concatenated to previously read `progfile`s
///
/// # Arguments
/// - `cmd_line_matches` the matched command line arguments provided by the user at runtime
///
/// # Return value
/// - The awk program to run
fn get_awk_program(cmd_line_matches: &ArgMatches) -> String {
    let mut program = String::new();

    if let Some(provided_awk_filepaths) = cmd_line_matches.values_of(PROGRAM_FILE_KEY) {
        for awk_filepath in provided_awk_filepaths {
            // TODO: Support for '-' as a special filename
            let contents = match fs::read_to_string(awk_filepath) {
                Ok(contents) => contents,
                Err(_) => panic!("{:?}", TempAwkReadFileError::FileDoesNotExist),
            };
            program.push_str(contents.as_str());
        }
    } else {
        program = cmd_line_matches.value_of(PROGRAM_KEY).unwrap().into();
    }
    program
}
