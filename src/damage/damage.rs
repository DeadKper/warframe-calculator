use strum_macros::{EnumIter, Display};

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, EnumIter, Display)]
pub enum Type {
    Base,
    Faction,
    Eclipse,
    Xata,

    Impact,
    Puncture,
    Slash,

    Heat,
    Cold,
    Electric,
    Toxin,

    Blast,
    Corrosive,
    Magnetic,
    Gas,
    Radiation,
    Viral,
}

impl Type {
    pub fn dot(self) -> bool {
        self == Type::Slash || self == Type::Heat || self == Type::Electric || self == Type::Gas
    }

    pub fn mult(self) -> bool {
        self < Type::Impact
    }

    pub fn ips(self) -> bool {
        self >= Type::Impact && self <= Type::Slash
    }

    pub fn elem(self) -> bool {
        self >= Type::Heat
    }

    pub fn mix(self) -> bool {
        self >= Type::Blast
    }
}