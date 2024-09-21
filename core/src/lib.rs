#![warn(missing_debug_implementations)]

mod choice;
mod strategy;

#[derive(Debug)]
pub struct Runtime {
    #[cfg(feature = "inter")]
    inter: jailbird_inter::Interpreter,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "inter")]
            inter: jailbird_inter::Interpreter::new(),
        }
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
