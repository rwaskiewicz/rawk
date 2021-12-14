//! Integration tests for the entrypoint of the CLI

mod utils;

#[cfg(test)]
mod entrypoint {
    use crate::utils;

    #[test]
    fn prints_version_info_when_the_version_flag_provided() {
        utils::run_rawk(
            None,
            vec!["-V"],
            None,
            Some(
                format!(
                    "{} version {}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
                )
                .as_str(),
            ),
        );
    }

    #[test]
    fn prints_version_info_neither_program_nor_file_flag_provided() {
        utils::run_rawk(
            None,
            vec![],
            None,
            Some(
                format!(
                    "{} version {}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
                )
                .as_str(),
            ),
        );
    }

    #[test]
    fn prints_nothing_when_empty_program_provided() {
        // mark this as a 'quick' test to prevent us from awaiting user input
        utils::run_rawk(Some(""), vec!["-q"], None, None);
    }
}
