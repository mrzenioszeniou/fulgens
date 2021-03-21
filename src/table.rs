use crate::column::Column;
use crate::data::Data;
use crate::error::Error;

pub struct Table {
  columns: Vec<Column>,
}

impl Table {

  pub fn new() -> Self {
    Table {
      columns: vec![],
    }
  }

  pub fn add_column(&mut self, column: Column) {
    self.columns.push(column);
  }

  pub fn add_row(mut self, row: Vec<Data>) -> Result<(), Error>{
    unimplemented!();
  }

}