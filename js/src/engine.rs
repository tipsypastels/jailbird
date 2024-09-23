use crate::{Context, Player, Turn};
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

    pub fn set_context(&mut self, ctx: impl Context) {
        let key = js_str!("__context__");
        let value = self.context(ctx);

        self.boa
            .global_object()
            .set(key, value, /* throw on err */ true, &mut self.boa)
            .expect("failed to set context global");
    }

    fn context(&mut self, ctx: impl Context) -> JsObject {
        let boa = &mut self.boa;
        let attr = Attribute::empty();

        let turn_ctx = ctx.turn();
        let turn_obj = ObjectInitializer::new(boa)
            .property(js_str!("cur"), turn_ctx.cur(), attr)
            .property(js_str!("max"), turn_ctx.max(), attr)
            .build();

        fn _player_context(boa: &mut Boa, player: impl Player) -> JsObject {
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

        let mut player_context = |p| _player_context(boa, p);

        let this_player_obj = player_context(ctx.this_player());
        let other_player_obj = player_context(ctx.other_player());

        ObjectInitializer::new(boa)
            .property(js_str!("turn"), turn_obj, attr)
            .property(js_str!("thisPlayer"), this_player_obj, attr)
            .property(js_str!("otherPlayer"), other_player_obj, attr)
            .build()
    }
}
