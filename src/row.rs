use crate::data::Data;
use std::iter::IntoIterator;

pub struct Row {
  values: Vec<Data>
}

impl Row {
  pub fn new() -> Self {
    Self {
      values: vec![],
    }
  }

  pub fn with(mut self, value: u64) -> Self {
    self.values.push(Data::from(value));
    self
  }

  pub fn len(&self) -> usize {
    self.values.len()
  }

}

impl IntoIterator for Row {

    type Item = Data;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}