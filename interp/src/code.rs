use crate::{ChoiceContext, Context, IString16, PlayerContext, TurnContext};
use boa_engine::{
    js_str, js_string,
    object::{builtins::JsArray, ObjectInitializer},
    property::Attribute,
    JsObject, JsStr, JsString, JsSymbol,
};
use implicit_clone::ImplicitClone;

// TODO: Consider persisting the boa state and making context a param.
#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub struct Code {
    function: JsString,
}

impl Code {
    pub fn new(body: IString16) -> Self {
        let prologue = JsStr::latin1(b"function main() {");
        let epilogue = JsStr::latin1(b"}");
        let function = js_string!(prologue, &body.inner, epilogue);
        Self { function }
    }

    pub fn interpret(&self, ctx: impl Context) -> boa_engine::JsValue {
        let mut boa = Boa::default();

        let cooperate = boa.register_choice_symbol("COOPERATE");
        let defect = boa.register_choice_symbol("DEFECT");

        boa.register_context(ctx, cooperate, defect);
        boa.run(self)
    }
}

#[derive(Default)]
struct Boa {
    engine: boa_engine::Context,
}

impl Boa {
    fn register_choice_symbol(&mut self, s: &'static str) -> JsSymbol {
        let key = JsStr::latin1(s.as_bytes());
        let sym =
            JsSymbol::new(Some(s.into())).unwrap_or_else(|| panic!("failed to make {s} symbol"));

        self.engine
            .register_global_property(key, sym.clone(), Attribute::empty())
            .unwrap_or_else(|_| panic!("duplicate {s} symbol"));

        sym
    }

    fn register_context(&mut self, ctx: impl Context, cooperate: JsSymbol, defect: JsSymbol) {
        let engine = &mut self.engine;
        let attr = Attribute::empty();

        let turn_ctx = ctx.turn();
        let turn_obj = ObjectInitializer::new(engine)
            .property(js_str!("current"), turn_ctx.current(), attr)
            .property(js_str!("total"), turn_ctx.total(), attr)
            .build();

        fn _make_player_ctx(
            engine: &mut boa_engine::Context,
            player: impl PlayerContext,
            cooperate: JsSymbol,
            defect: JsSymbol,
        ) -> JsObject {
            let elements = player
                .choices()
                .iter()
                .map(|c| {
                    if c.is_cooperate() {
                        cooperate.clone()
                    } else {
                        defect.clone()
                    }
                })
                .map(Into::into);

            let choices = JsArray::from_iter(elements, engine);

            ObjectInitializer::new(engine)
                .property(js_str!("choices"), choices, Attribute::empty())
                .build()
        }

        let mut make_player_ctx =
            |player| _make_player_ctx(engine, player, cooperate.clone(), defect.clone());

        let this_player_ctx = make_player_ctx(ctx.this_player());
        let other_player_ctx = make_player_ctx(ctx.other_player());

        let obj = ObjectInitializer::new(engine)
            .property(js_str!("turn"), turn_obj, attr)
            .property(js_str!("thisPlayer"), this_player_ctx, attr)
            .property(js_str!("otherPlayer"), other_player_ctx, attr)
            .build();

        engine
            .register_global_property(js_str!("context"), obj, attr)
            .expect("duplicate context");
    }

    fn run(mut self, code: &Code) -> boa_engine::JsValue {
        use boa_engine::Source;
        // TODO: We really shouldn't have to allocate here, right?
        // How to get &[u16] off this?
        let function = code.function.to_vec();
        let function = Source::from_utf16(&function);
        self.engine
            .eval(function)
            .expect("failed to define main function");

        let main = Source::from_bytes(b"main();");
        self.engine
            .eval(main)
            .expect("failed to evaluate main function")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let code = Code::new(IString16 {
            inner: js_string!("return DEFECT;"),
        });

        impl Context for () {
            type Turn = ();
            type Player = ();

            fn turn(&self) {}
            fn this_player(&self) {}
            fn other_player(&self) {}
        }

        impl TurnContext for () {
            fn current(&self) -> i32 {
                0
            }

            fn total(&self) -> i32 {
                1
            }
        }

        impl PlayerContext for () {
            type Choice = ();

            fn choices(&self) -> &[Self::Choice] {
                const { &[()] }
            }
        }

        impl ChoiceContext for () {
            fn is_cooperate(self) -> bool {
                true
            }
        }

        assert!(code.interpret(()).is_symbol());
    }
}
