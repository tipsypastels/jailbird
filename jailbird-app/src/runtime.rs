use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};
use yew::ContextProvider;

pub type RuntimeContext = Rc<RefCell<Runtime>>;
pub type RuntimeContextProvider = ContextProvider<RuntimeContext>;

#[derive(Debug)]
pub struct Runtime(jailbird::Runtime);

impl Runtime {
    pub fn new() -> Self {
        Self(jailbird::Runtime::new())
    }
}

impl PartialEq for Runtime {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Deref for Runtime {
    type Target = jailbird::Runtime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Runtime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
