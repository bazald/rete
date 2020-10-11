use crate::float::Float;
use core::fmt;

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
pub enum Symbol {
  Integer(i64),
  Float(Float),
  String(String),
  Identifier(String),
  Variable(String),
}

impl fmt::Display for Symbol {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Symbol::Integer(ii) => write!(f, "{}", ii),
      Symbol::Float(ff) => write!(f, "{}", ff),
      Symbol::String(ss) => write!(f, "\"{}\"", ss),
      Symbol::Identifier(id) => write!(f, ">{}<", id),
      Symbol::Variable(va) => write!(f, "<{}>", va),
    }
  }
}

impl From<i64> for Symbol {
  fn from(value: i64) -> Self {
    Symbol::Integer(value)
  }
}

impl From<f64> for Symbol {
  fn from(value: f64) -> Self {
    Symbol::Float(value.into())
  }
}

impl From<&str> for Symbol {
  fn from(value: &str) -> Self {
    Symbol::String(value.into())
  }
}
impl From<String> for Symbol {
  fn from(value: String) -> Self {
    Symbol::String(value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn display_symbols() {
    let ii_sym = Symbol::from(42);
    assert_eq!(ii_sym.to_string(), "42");
    
    let ss_sym = Symbol::from("Hello, world!");
    assert_eq!(ss_sym.to_string(), "\"Hello, world!\"");
    
    let id_sym = Symbol::Identifier("HW".into());
    assert_eq!(id_sym.to_string(), ">HW<");
    
    let va_sym = Symbol::Variable("hello-world".into());
    assert_eq!(va_sym.to_string(), "<hello-world>");
  }
}
