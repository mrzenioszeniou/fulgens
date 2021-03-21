use crate::data::Data;

pub struct Column {
  values: Vec<Data>,
}

impl Column {

  pub fn new() -> Self {
    Column {
      values: vec![],
    }
  }

}