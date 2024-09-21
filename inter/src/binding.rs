use crate::engine::Engine;
use boa_engine::{object::builtins::JsFunction, JsError, JsValue};
use implicit_clone::ImplicitClone;
use jailbird_choice::Choice;
use std::{error::Error, fmt};

#[derive(Debug, Clone, ImplicitClone)]
pub struct Binding {
    function: JsFunction,
}

impl Binding {
    pub(crate) fn new(body: &str, engine: &mut Engine) -> Self {
        let code = format!("() => {{const context = globalThis.__context__;{body}}}");
        let function = engine.init_function(&code);

        Self { function }
    }

    pub(crate) fn call(&self, engine: &mut Engine) -> CallResult {
        let value = engine
            .call_function(&self.function)
            .map_err(CallError::ThrownError)?;

        let choice = match value {
            JsValue::Boolean(b) => Choice::from_bool(b),
            value => return Err(CallError::ExpectedChoice { got_value: value }),
        };

        Ok(choice)
    }
}

// JsObject impls this but JsFunction which derefs to it does not for some reason.
impl PartialEq for Binding {
    fn eq(&self, other: &Self) -> bool {
        *self.function == *other.function
    }
}

pub type CallResult = Result<Choice, CallError>;

#[derive(Debug, Clone)]
pub enum CallError {
    ExpectedChoice { got_value: JsValue },
    ThrownError(JsError),
}

impl fmt::Display for CallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedChoice { got_value } => write!(f, "Expected choice, got {got_value:?}"),
            Self::ThrownError(error) => write!(f, "Failed to call binding: {error}"),
        }
    }
}

impl Error for CallError {}
