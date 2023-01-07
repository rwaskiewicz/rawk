use clap::{parser::ValueSource, Arg, ArgAction, ArgMatches, Command};
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
const DATA_FILE: &str = "data_file";
const QUICK_KEY: &str = "quick";
const EVAL_KEY: &str = "eval";
const FIELD_SEPARATOR_KEY: &str = "field_separator";

fn main() -> Result<(), Box<dyn Error>> {
    Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .filter_module("rustyline", LevelFilter::Error)
        .init();

    let cmd = build_awk_cli_command();
    let render_version = cmd.render_version();
    let cmd_line_matches = cmd.get_matches();

    let has_program_literal = cmd_line_matches.contains_id(PROGRAM_KEY);
    let has_program_file = cmd_line_matches.contains_id(PROGRAM_FILE_KEY);
    // catch the case where clap believes we've found a program literal and a program file:
    // $ rawk -f some_awk.awk '{print $0;}'
    // but really we've found:
    // $ rawk -f some_awk.awk some_data.dat
    // the problem being it's hard to disambiguate whether the first positional argument is an awk
    // program or a file on disk to read
    let is_program_data_file = has_program_literal && has_program_file;

    if !has_program_literal && !has_program_file {
        // clap will add a newline to the rendered version string for us
        print!("{render_version}");
        return Ok(());
    }

    let field_separator = cmd_line_matches
        .get_one::<String>(FIELD_SEPARATOR_KEY)
        .map(|separator| separator.to_string())
        .unwrap_or_else(|| panic!("{} not configured for command line", FIELD_SEPARATOR_KEY));

    let is_eval = cmd_line_matches
        .value_source(EVAL_KEY)
        .unwrap_or_else(|| panic!("{} not configured for command line", EVAL_KEY))
        .eq(&ValueSource::CommandLine);
    let is_quick = cmd_line_matches
        .value_source(QUICK_KEY)
        .unwrap_or_else(|| panic!("{} not configured for command line", QUICK_KEY))
        .eq(&ValueSource::CommandLine);

    let mut data_file_paths: Vec<String> = if is_program_data_file {
        vec![cmd_line_matches
            .get_one::<String>(PROGRAM_KEY)
            .unwrap()
            .into()]
    } else {
        Vec::new()
    };

    cmd_line_matches
        .get_many::<String>(DATA_FILE)
        .unwrap_or_default()
        .cloned()
        .for_each(|data_file_path| {
            data_file_paths.push(data_file_path);
        });

    let config: RuntimeConfig =
        RuntimeConfig::new(data_file_paths, field_separator, is_eval, is_quick, None);

    let program = get_awk_program(&cmd_line_matches);
    rawk::run_program(&program, config);
    Ok(())
}

fn build_awk_cli_command() -> Command {
    // https://www.gnu.org/software/gawk/manual/html_node/Options.html
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("awk, implemented in Rust")
        .arg(
            Arg::new(PROGRAM_FILE_KEY)
                .short('f')
                .long(PROGRAM_FILE_KEY)
                .num_args(1)
                .required(false)
                .action(ArgAction::Append)
                .help("Runs an awk program"),
        )
        .arg(
            // note this is the first positional argument relative to other positional arguments.
            // it is not the first position in the argument list as a whole
            Arg::new(PROGRAM_KEY).index(1),
        )
        .arg(
            // note this is the second positional argument relative to other positional arguments.
            // it is not the second position in the argument list as a whole
            Arg::new(DATA_FILE).index(2).num_args(1..),
        )
        .arg(
            // TODO: Remove this when `BEGIN` is implemented. We could use -w, but this is quicker
            Arg::new(QUICK_KEY)
                .short('q')
                .long(QUICK_KEY)
                .action(ArgAction::SetTrue)
                .help("Runs a single line of awk code without data, then terminates"),
        )
        .arg(
            Arg::new(EVAL_KEY)
                .short('k') // '-e' is taken already...
                .long(EVAL_KEY)
                .action(ArgAction::SetTrue)
                .help("Runs a single line of awk code, then terminates"),
        )
        .arg(
            Arg::new(FIELD_SEPARATOR_KEY)
                .short('F')
                .num_args(1)
                .required(false)
                .action(ArgAction::Set)
                .default_value(" ")
                .help("Sets the field separator character/regex for parsing data"),
        )
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
    if let Some(provided_awk_filepaths) = cmd_line_matches.get_many::<String>(PROGRAM_FILE_KEY) {
        provided_awk_filepaths
            .map(|awk_filepath| {
                // TODO: Support for '-' as a special filename
                match fs::read_to_string(awk_filepath) {
                    Ok(contents) => contents,
                    Err(_) => panic!("{:?}", TempAwkReadFileError::FileDoesNotExist),
                }
            })
            .collect()
    } else {
        cmd_line_matches
            .get_one::<String>(PROGRAM_KEY)
            .cloned()
            .unwrap()
    }
}

#[cfg(test)]
mod cli {
    use crate::build_awk_cli_command;

    #[test]
    fn verify_cli() {
        build_awk_cli_command().debug_assert();
    }
}
