extern crate fulgens;

use crate::fulgens::table::Table;
use crate::fulgens::column::Column;
use crate::fulgens::row::Row;

#[test]
fn main() {

  let mut table = Table::new();

  let column_a = Column::new("A");
  let column_b = Column::new("B");

  table.add_column(column_a);
  table.add_column(column_b);

  table.add_row(Row::new().with(13).with(2));
  println!("{}", table);
}