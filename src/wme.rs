use crate::symbol::*;
use std::fmt;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WmeIndex {
    Identifier,
    Attribute,
    Value,
}
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct Wme {
    pub id: Symbol,
    pub attr: Symbol,
    pub value: Symbol,
}

#[allow(dead_code)]
impl Wme {
    pub fn new(id: Symbol, attr: Symbol, value: Symbol) -> Self {
        Wme {
            id,
            attr,
            value,
        }
    }

    pub fn at(&self, index: WmeIndex) -> &'_ Symbol {
        match index {
            WmeIndex::Identifier => &self.id,
            WmeIndex::Attribute => &self.attr,
            WmeIndex::Value => &self.value,
        }
    }
}

impl fmt::Display for Wme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.id, self.attr, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_wme() {
        let wme = Wme::new(42.into(), "Hello, world!".into(), Symbol::Identifier("hello-world".into()));
        assert_eq!(wme.to_string(), "(42, \"Hello, world!\", [hello-world])");
    }
}
