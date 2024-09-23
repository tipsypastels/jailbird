use crate::engine::Engine;
use boa_engine::{object::builtins::JsFunction, JsError, JsValue};
use implicit_clone::{unsync::IString, ImplicitClone};
use jailbird_choice::Choice;
use std::{error::Error, fmt};

#[derive(Debug, Clone, ImplicitClone)]
pub struct Function {
    body: IString,
    binding: JsFunction,
}

impl Function {
    pub(crate) fn new(body: &str, engine: &mut Engine) -> Self {
        let code = format!("() => {{const context = globalThis.__context__;{body}}}");
        let binding = engine.init_function(&code);
        let body = body.to_string().into();

        Self { body, binding }
    }

    pub(crate) fn call(&self, engine: &mut Engine) -> CallResult {
        let value = engine
            .call_function(&self.binding)
            .map_err(CallError::ThrownError)?;

        let choice = match value {
            JsValue::Boolean(b) => Choice::from_bool(b),
            value => return Err(CallError::ExpectedChoice { got_value: value }),
        };

        Ok(choice)
    }

    pub fn body(&self) -> IString {
        self.body.clone()
    }
}

// JsObject impls this but JsFunction which derefs to it does not for some reason.
impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body && *self.binding == *other.binding
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
