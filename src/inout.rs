use std::fmt::{Display, Formatter, Result};

use crate::utils::{Collected, FormattedSequence};

#[derive(Debug)]
pub struct FStatInput {
    pub options: Vec<String>,
    pub ignore_objects: Vec<String>,
    pub file_extensions: Vec<String>,
    pub other_arguments: Vec<String>
}

impl FStatInput {
    pub fn from(args: Vec<String>) -> Self {
        let mut options = Vec::new();
        let mut ignore_objects = Vec::new();
        let mut file_extensions = Vec::new();
        let mut other_arguments = Vec::new();

        let args_i = args.iter()
            .skip(1);

        for arg in args_i {
            let argument = arg.to_string();

            if arg.starts_with("--") {
                options.push(argument);
            } else if arg.starts_with(".") {
                file_extensions.push(argument);
            } else if arg.starts_with("/") {
                ignore_objects.push(argument);
            } else {
                other_arguments.push(argument);
            }
        }

        FStatInput {
            options,
            ignore_objects,
            file_extensions,
            other_arguments
        }
    }
}

#[derive(Debug)]
pub struct FStatOutput<'a> {
    pub collected: Collected,
    pub file_extensions: &'a [String]
}

impl<'a> From<&'a FStatInput> for FStatOutput<'a> {
    fn from(input: &'a FStatInput) -> Self {
        FStatOutput {
            collected: Collected::new(),
            file_extensions: input.file_extensions.as_slice()
        }
    }
}

impl Display for FStatOutput<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let files_seq = FormattedSequence(self.file_extensions);
        let mut base = format!(
            "Files with '{}' extensions: {}.\n", files_seq, self.collected.files
        );

        base.push_str(&format!(
            "Total lines of code: {}.\n", self.collected.lines
        ));
        base.push_str(&format!(
            "Folders found: {}.", self.collected.folders
        ));

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
        let _pseudo_output = FStatOutput::from(&pseudo_input);
    }
}
