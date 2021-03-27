
use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum Data {
  
  UInt {
    value: u64, 
  },

  Int {
    value: i64,
  },

  Float {
    value: f64,
  },

  DateTime {
    value: DateTime<Utc>,
  }
}

impl Data {

  pub fn same_type(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::UInt { .. }, Self::UInt { .. })
      | (Self::Int { .. }, Self::Int { .. })
      | (Self::Float { .. }, Self::Float { .. }) 
      | (Self::DateTime { .. }, Self::DateTime { .. }) => true,
      _ => false
    }
  }

}

impl From<u64> for Data {
  fn from(from: u64) -> Self {
    Self::UInt {
      value: from,
    }
  }
}

impl Display for Data {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
      match self {
        Self::Int { value }=> Display::fmt(value, f),
        Self::UInt { value }=> Display::fmt(value, f),
        Self::Float { value }=> Display::fmt(value, f),
        Self::DateTime { value }=> Display::fmt(value, f),
      }
  }
}