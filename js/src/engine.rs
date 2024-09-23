use crate::{PlayerView, TurnView, View};
use boa_engine::{
    js_str,
    object::{
        builtins::{JsArray, JsFunction},
        ObjectInitializer,
    },
    property::Attribute,
    Context as Boa, JsObject, JsResult, JsStr, JsValue, Source,
};

#[derive(Debug)]
pub struct Engine {
    boa: Boa,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            boa: Boa::default(),
        }
    }

    pub fn init_choice(&mut self, key: JsStr<'static>, b: bool) {
        self.boa
            .register_global_property(key, b, Attribute::empty())
            .expect("duplicate choice");
    }

    pub fn init_function(&mut self, code: &str) -> JsFunction {
        self.boa
            .eval(Source::from_bytes(code.as_bytes()))
            .expect("failed to init function")
            .try_js_into(&mut self.boa)
            .expect("init_function did not return function")
    }

    pub fn call_function(&mut self, function: &JsFunction) -> JsResult<JsValue> {
        function.call(&JsValue::Undefined, &[], &mut self.boa)
    }

    pub fn set_view(&mut self, view: impl View) {
        let key = js_str!("__view__");
        let value = self.view(view);

        self.boa
            .global_object()
            .set(key, value, /* throw on err */ true, &mut self.boa)
            .expect("failed to set view global");
    }

    fn view(&mut self, view: impl View) -> JsObject {
        let boa = &mut self.boa;
        let attr = Attribute::empty();

        let turn = view.turn();
        let turn_obj = ObjectInitializer::new(boa)
            .property(js_str!("cur"), turn.cur(), attr)
            .property(js_str!("max"), turn.max(), attr)
            .build();

        fn player_view(boa: &mut Boa, player: impl PlayerView) -> JsObject {
            let attr = Attribute::empty();
            let elements = player
                .choices()
                .iter()
                .map(|&c| JsValue::Boolean(c.is_cooperate()));

            let choices = JsArray::from_iter(elements, boa);

            let score = player.score();
            let ever_c = player.ever_cooperated();
            let ever_d = player.ever_defected();

            ObjectInitializer::new(boa)
                .property(js_str!("score"), score, attr)
                .property(js_str!("choices"), choices, attr)
                .property(js_str!("everCooperated"), ever_c, attr)
                .property(js_str!("everDefected"), ever_d, attr)
                .build()
        }

        let this_player_obj = player_view(boa, view.this_player());
        let other_player_obj = player_view(boa, view.other_player());

        ObjectInitializer::new(boa)
            .property(js_str!("turn"), turn_obj, attr)
            .property(js_str!("thisPlayer"), this_player_obj, attr)
            .property(js_str!("otherPlayer"), other_player_obj, attr)
            .build()
    }
}
