
use chrono::{DateTime, Utc};

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