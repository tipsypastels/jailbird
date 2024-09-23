use implicit_clone::ImplicitClone;

#[derive(Debug, Clone, ImplicitClone, PartialEq)]
#[non_exhaustive]
pub struct Turn {
    pub current: u32,
    pub total: u32,
}

impl Turn {
    pub fn new(total: u32) -> Self {
        Self { current: 0, total }
    }

    pub fn done(&self) -> bool {
        self.current == self.total
    }

    pub fn next(&self) -> Option<Self> {
        (!self.done()).then(|| Self {
            current: self.current + 1,
            total: self.total,
        })
    }
}

impl From<u32> for Turn {
    fn from(total: u32) -> Self {
        Self::new(total)
    }
}

#[cfg(feature = "inter")]
impl jailbird_inter::Turn for Turn {
    fn cur(&self) -> i32 {
        self.current as i32
    }

    fn max(&self) -> i32 {
        self.total as i32
    }
}
