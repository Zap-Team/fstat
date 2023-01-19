use std::fmt::{Debug, Display, Write};

pub enum Target {
    Files,
    Lines,
    Folders
}

#[derive(Debug)]
pub struct Collected {
    pub files: usize,
    pub lines: usize,
    pub folders: usize
}

impl Collected {
    pub fn new() -> Self {
        Collected {
            files: 0,
            lines: 0,
            folders: 0
        }
    }

    pub fn append(&mut self, target: Target, value: usize) {
        match target {
            Target::Files => self.files += value,
            Target::Lines => self.lines += value,
            Target::Folders => self.folders += value
        }
    }
}

pub fn format_sequence<T: Display>(vector: Vec<T>) -> String {
    if vector.len() <= 0 {
        return String::new();
    }

    let mut fmt = format!("{}", vector[0]);
    let vector_i = vector.iter()
        .skip(1);

    for element in vector_i {
        write!(fmt, ", {element}")
            .expect("failed to format the sequence.");
    }

    fmt
}

#[cfg(test)]
mod tests {
    use super::{Collected, format_sequence};

    fn get_pseudo_collected() -> Collected {
        use super::Target::*;
        let mut collected = Collected::new();

        collected.append(Files, 3);
        collected.append(Lines, 900);
        collected.append(Folders, 2);

        collected
    }

    #[test]
    fn collected_test() {
        let collected = get_pseudo_collected();

        assert_eq!(collected.files, 3);
        assert_eq!(collected.lines, 900);
        assert_eq!(collected.folders, 2);
    }

    #[test]
    fn format_sequence_test() {
        let empty_vec: Vec<String> = Vec::new();

        let empty_seq = format_sequence(empty_vec);
        let single_seq = format_sequence(vec!["Hi!"]);
        let multiple_seq = format_sequence(vec![1, 2, 3]);

        assert_eq!(empty_seq, String::new());
        assert_eq!(single_seq, String::from("Hi!"));
        assert_eq!(multiple_seq, String::from("1, 2, 3"));
    }
}
