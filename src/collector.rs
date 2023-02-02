use crate::inout::{FStatInput, FStatOutput};
// use crate::utils::{Target};

pub struct FStatCollector {
    input: FStatInput
}

impl From<FStatInput> for FStatCollector {
    fn from(input: FStatInput) -> Self {
        FStatCollector { 
            input 
        }
    }
}

impl FStatCollector {
    pub fn collect(&self) -> FStatOutput {
        FStatOutput::from(&self.input)
    }
}
