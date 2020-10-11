use core::{cmp::PartialEq, fmt, hash::{Hash, Hasher}};

#[derive(Clone, Debug, PartialOrd)]
pub struct Float {
  value: f64,
}

impl fmt::Display for Float {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.value)
  }
}

impl From<f64> for Float {
  fn from(value: f64) -> Self {
    Float {
      value,
    }
  }
}

impl From<Float> for f64 {
  fn from(value: Float) -> Self {
    value.value
  }
}

impl Hash for Float {
  fn hash<H: Hasher>(&self, state: &mut H) {
      (self.value as i64).hash(state);
  }
}

impl PartialEq for Float {
  fn eq(&self, other: &Self) -> bool {
    (self.value as i64) == (other.value as i64)
  }
}
