use crate::{engine::Engine, ChoiceContext};
use boa_engine::{js_str, JsSymbol, JsValue};

#[derive(Debug, Clone)]
pub struct Outcomes {
    cooperate: JsSymbol,
    defect: JsSymbol,
}

impl Outcomes {
    pub fn new(engine: &mut Engine) -> Self {
        let cooperate = engine.init_outcome(js_str!("COOPERATE"));
        let defect = engine.init_outcome(js_str!("DEFECT"));
        Self { cooperate, defect }
    }

    pub fn value_from_choice(&self, cc: &impl ChoiceContext) -> JsValue {
        if cc.is_cooperate() {
            &self.cooperate
        } else {
            &self.defect
        }
        .clone()
        .into()
    }

    pub fn choice_from_value<Cc: ChoiceContext>(&self, value: JsValue) -> Result<Cc, JsValue> {
        if JsValue::Symbol(self.cooperate.clone()) == value {
            return Ok(Cc::cooperate());
        } else if JsValue::Symbol(self.defect.clone()) == value {
            return Ok(Cc::defect());
        }
        Err(value)
    }
}
