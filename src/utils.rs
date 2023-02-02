use std::fmt::{
    Debug, 
    Display, 
    Formatter, 
    Result
};

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

pub struct FormattedSequence<'a, T>(pub &'a [T]);

impl<T: Display> Display for FormattedSequence<'_, T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.0 {
            [] => Ok(()),
            [base, rest @ ..] => {
                write!(f, "{base}")?;

                for item in rest {
                    write!(f, ", {item}")?;
                }

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Target, Collected, FormattedSequence};

    fn get_pseudo_collected() -> Collected {
        let mut collected = Collected::new();

        collected.append(Target::Files, 3);
        collected.append(Target::Lines, 900);
        collected.append(Target::Folders, 2);

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
        let empty_seq = FormattedSequence::<String>(&[]);
        let single_seq = FormattedSequence(&["Hi!"]);
        let multiple_seq = FormattedSequence(&[1, 2, 3]);

        assert_eq!(empty_seq.to_string(), String::new());
        assert_eq!(single_seq.to_string(), String::from("Hi!"));
        assert_eq!(multiple_seq.to_string(), String::from("1, 2, 3"));
    }
}
