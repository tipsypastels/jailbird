use crate::{
    function::{Function, FunctionInner, NativeFunction},
    Runtime,
};
use implicit_clone::{unsync::IString, ImplicitClone};

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
pub struct Strategy {
    name: IString,
    desc: Option<IString>,
    func: Function,
}

impl Strategy {
    pub fn name(&self) -> IString {
        self.name.clone()
    }

    pub fn desc(&self) -> Option<IString> {
        self.desc.clone()
    }
}

#[derive(Debug)]
pub struct StrategyBuilder<'rt, Step> {
    rt: &'rt mut Runtime,
    desc: Option<IString>,
    step: Step,
}

impl<Step> StrategyBuilder<'_, Step> {
    pub fn desc(self, desc: impl Into<IString>) -> Self {
        Self {
            rt: self.rt,
            desc: Some(desc.into()),
            step: self.step,
        }
    }
}

impl<'rt> StrategyBuilder<'rt, Step0> {
    pub(crate) fn new(rt: &'rt mut Runtime) -> Self {
        Self {
            rt,
            desc: None,
            step: Step0,
        }
    }

    pub fn name(self, name: impl Into<IString>) -> StrategyBuilder<'rt, Step1> {
        StrategyBuilder {
            rt: self.rt,
            desc: self.desc,
            step: Step1 { name: name.into() },
        }
    }
}

impl<'rt> StrategyBuilder<'rt, Step1> {
    pub fn native(self, func: NativeFunction) -> StrategyBuilder<'rt, Step2> {
        StrategyBuilder {
            rt: self.rt,
            desc: self.desc,
            step: Step2 {
                name: self.step.name,
                func: Function(FunctionInner::Native(func)),
            },
        }
    }

    #[cfg(feature = "js")]
    pub fn native_with_js_example(
        self,
        func: NativeFunction,
        example: &'static str,
    ) -> StrategyBuilder<'rt, Step2> {
        StrategyBuilder {
            rt: self.rt,
            desc: self.desc,
            step: Step2 {
                name: self.step.name,
                func: Function(FunctionInner::NativeWithJsExample { func, example }),
            },
        }
    }

    #[cfg(feature = "js")]
    pub fn js(self, code: &str) -> StrategyBuilder<'rt, Step2> {
        let func = self.rt.js.bind(code);
        StrategyBuilder {
            rt: self.rt,
            desc: self.desc,
            step: Step2 {
                name: self.step.name,
                func: Function(FunctionInner::Js(func)),
            },
        }
    }

    #[cfg(feature = "js")]
    pub(crate) fn builtin(
        self,
        func: NativeFunction,
        example: &'static str,
    ) -> StrategyBuilder<'rt, Step2> {
        self.native_with_js_example(func, example)
    }

    #[cfg(not(feature = "js"))]
    pub(crate) fn builtin(
        self,
        func: NativeFunction,
        _: &'static str,
    ) -> StrategyBuilder<'rt, Step2> {
        self.native(func)
    }
}

impl<'rt> StrategyBuilder<'rt, Step2> {
    pub fn build(self) -> Strategy {
        Strategy {
            name: self.step.name,
            desc: self.desc,
            func: self.step.func,
        }
    }
}

#[derive(Debug)]
pub struct Step0;
#[derive(Debug)]
pub struct Step1 {
    name: IString,
}
#[derive(Debug)]
pub struct Step2 {
    name: IString,
    func: Function,
}
