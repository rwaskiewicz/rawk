//! Runtime configuration structs and impl

pub struct RuntimeConfig {
    // the name of the awk file to read/run
    pub file_name: Option<String>,
    // a single character or regex to be used to split user input by
    pub field_separator: String,
    // whether or not a single line of awk code is being interpreted. If so, terminate after a single line of code from
    // STDIN has been evaluated.
    pub is_eval: bool,
    // whether or not a single line of awk code is being interpreted without data. If so, terminate after no code from
    // STDIN has been evaluated and is a temporary stand-in for `BEGIN`.
    pub is_quick: bool,
}

impl RuntimeConfig {
    pub fn new(
        file_name: Option<String>,
        field_separator: String,
        is_eval: bool,
        is_quick: bool,
    ) -> RuntimeConfig {
        RuntimeConfig {
            file_name,
            field_separator,
            is_eval,
            is_quick,
        }
    }
}
