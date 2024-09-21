use crate::{engine::Engine, outcomes::Outcomes, Context, PlayerContext};
use boa_engine::{object::builtins::JsFunction, JsError, JsValue};
use implicit_clone::ImplicitClone;
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

    pub(crate) fn call<C>(&self, outcomes: &Outcomes, engine: &mut Engine) -> CallResult<C>
    where
        C: Context,
    {
        let value = engine
            .call_function(&self.function)
            .map_err(CallError::ThrownError)?;

        let choice = outcomes
            .choice_from_value::<<C::Player as PlayerContext>::Choice>(value)
            .map_err(CallError::ExpectedChoice)?;

        Ok(choice)
    }
}

// JsObject impls this but JsFunction which derefs to it does not for some reason.
impl PartialEq for Binding {
    fn eq(&self, other: &Self) -> bool {
        *self.function == *other.function
    }
}

pub type CallResult<C> = Result<<<C as Context>::Player as PlayerContext>::Choice, CallError>;

#[derive(Debug, Clone)]
pub enum CallError {
    ExpectedChoice(JsValue),
    ThrownError(JsError),
}

impl fmt::Display for CallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedChoice(value) => write!(f, "Expected choice, got {value:?}"),
            Self::ThrownError(error) => write!(f, "Failed to call binding: {error}"),
        }
    }
}

impl Error for CallError {}
