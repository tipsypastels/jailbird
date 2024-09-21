use implicit_clone::{unsync::IString, ImplicitClone};

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub struct Strategy {
    name: IString,
    desc: Option<IString>,
    body: StrategyBody,
}

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
enum StrategyBody {
    Func(fn()),
    #[cfg(feature = "inter")]
    Inter(jailbird_inter::Binding),
}

pub struct StrategyBuilder<State> {
    name: IString,
    desc: Option<IString>,
    state: State,
}
