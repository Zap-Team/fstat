use std::fmt::{Display, Formatter, Result};

use crate::traits::FmtOutputBack;
use crate::utils::Collected;

#[derive(Debug)]
pub struct FStatInput {
    pub options: Vec<String>,
    pub ignore_objects: Vec<String>,
    pub file_extensions: Vec<String>
}

impl FStatInput {
    pub fn from(mut args: Vec<String>) -> Self {
        let mut options = Vec::new();
        let mut ignore_objects = Vec::new();
        let mut file_extensions = Vec::new();

        args.remove(0);

        for arg in args {
            if arg.starts_with("--") {
                options.push(arg);
            } else if arg.starts_with(".") {
                file_extensions.push(arg);
            } else {
                ignore_objects.push(arg);
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
    pub fn from(input: FStatInput) -> Self {
        FStatOutput {
            input,
            collected: Collected::new()
        }
    }
}

impl FmtOutputBack for FStatOutput {
    fn get_pretty(&self) -> String {
        todo!()
    }

    fn format_fields(&self) -> String {
        todo!()
    }
}

impl Display for FStatOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let pretty_output = self.get_pretty();
        f.write_str(pretty_output.as_str())
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
