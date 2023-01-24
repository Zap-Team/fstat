use std::fmt::{Display, Formatter, Result};

use crate::utils::{Collected, format_sequence};

#[derive(Debug)]
pub struct FStatInput {
    pub options: Vec<String>,
    pub ignore_objects: Vec<String>,
    pub file_extensions: Vec<String>
}

impl FStatInput {
    pub fn from(args: Vec<String>) -> Self {
        let mut options = Vec::new();
        let mut ignore_objects = Vec::new();
        let mut file_extensions = Vec::new();

        let args_i = args.iter()
            .skip(1);

        for arg in args_i {
            let argument = arg.to_string();

            if arg.starts_with("--") {
                options.push(argument);
            } else if arg.starts_with(".") {
                file_extensions.push(argument);
            } else {
                ignore_objects.push(argument);
            }
        }

        FStatInput {
            options,
            ignore_objects,
            file_extensions
        }
    }
}

#[derive(Debug)]
pub struct FStatOutput {
    pub input: FStatInput,
    pub collected: Collected
}

impl FStatOutput {
    // FIXME: Panics cause it hides by some other `from`.
    pub fn from(input: FStatInput) -> Self {
        FStatOutput {
            input,
            collected: Collected::new()
        }
    }
}

trait Fmt {
    type FmtTuple;

    fn get_fmt_parts(&self) -> Self::FmtTuple;
}

impl Fmt for FStatOutput {
    type FmtTuple = (String, String, String);

    fn get_fmt_parts(&self) -> Self::FmtTuple {
        let files_seq = format_sequence(self.input.file_extensions.clone());

        let fmt_l1 = format!(
            "Files with '{files_seq}' extensions: {}.\n", self.collected.files
        );
        let fmt_l2 = format!(
            "Total lines of code: {}.\n", self.collected.lines
        );
        let fmt_l3 = format!(
            "Folders found: {}.", self.collected.folders
        );

        (fmt_l1, fmt_l2, fmt_l3)
    }
}

impl Display for FStatOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (mut base, fmt_p1, fmt_p2) = self.get_fmt_parts();

        base.push_str(&fmt_p1);
        base.push_str(&fmt_p2);

        f.write_str(&base)
    }
}

#[cfg(test)]
mod tests {
    use super::{FStatInput, FStatOutput};

    fn get_pseudo_input() -> FStatInput {
        let args = vec![
            String::from("fstat"),
            String::from("--verbose"),
            String::from(".rs"),
            String::from(".xml"),
            String::from(".iml"),
            String::from("/.idea")
        ];

        FStatInput::from(args)
    }

    #[test]
    fn fstat_input_test() {
        let pseudo_input = get_pseudo_input();

        assert_eq!(pseudo_input.options.len(), 1);
        assert_eq!(pseudo_input.options[0], String::from("--verbose"));

        assert_eq!(pseudo_input.ignore_objects.len(), 1);
        assert_eq!(pseudo_input.ignore_objects[0], String::from("/.idea"));

        assert_eq!(pseudo_input.file_extensions.len(), 3);
        assert_eq!(pseudo_input.file_extensions[0], String::from(".rs"));
        assert_eq!(pseudo_input.file_extensions[1], String::from(".xml"));
        assert_eq!(pseudo_input.file_extensions[2], String::from(".iml"));
    }

    #[test]
    fn fstat_output_test() {
        let pseudo_input = get_pseudo_input();
        let _pseudo_output = FStatOutput::from(pseudo_input);
    }
}
