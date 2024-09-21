use crate::{engine::Engine, outcomes::Outcomes, Context, PlayerContext};
use boa_engine::{object::builtins::JsFunction, JsError, JsValue};
use implicit_clone::ImplicitClone;

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
            .map_err(CallError::ExpectedCooperateOrDefect)?;

        Ok(choice)
    }
}

pub type CallResult<C> = Result<<<C as Context>::Player as PlayerContext>::Choice, CallError>;

#[derive(Debug, Clone)]
pub enum CallError {
    ExpectedCooperateOrDefect(JsValue),
    ThrownError(JsError),
}
