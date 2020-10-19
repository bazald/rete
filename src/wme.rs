use crate::symbol::*;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WMEIndex {
    Identifier,
    Attribute,
    Value,
}
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct WME {
    pub id: Symbol,
    pub attr: Symbol,
    pub value: Symbol,
}

impl WME {
    pub fn new(id: Symbol, attr: Symbol, value: Symbol) -> WME {
        WME {
            id,
            attr,
            value,
        }
    }

    pub fn at(&self, index: WMEIndex) -> &'_ Symbol {
        match index {
            WMEIndex::Identifier => &self.id,
            WMEIndex::Attribute => &self.attr,
            WMEIndex::Value => &self.value,
        }
    }
}

impl fmt::Display for WME {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.id, self.attr, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_wme() {
        let wme = WME::new(42.into(), "Hello, world!".into(), Symbol::Variable("hello-world".into()));
        assert_eq!(wme.to_string(), "(42, \"Hello, world!\", <hello-world>)");
    }
}
