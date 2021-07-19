extern crate fulgens;

use crate::fulgens::column::Column;
use crate::fulgens::data::Data;
use crate::fulgens::row::Row;
use crate::fulgens::table::Table;

#[test]
fn main() {
    let mut table = Table::new();

    let column_a = Column::new("A");
    let column_b = Column::new("B");
    let column_c = Column::new("C");

    table.add_column(column_a);
    table.add_column(column_b);
    table.add_column(column_c);

    table.add_row(
        Row::new()
            .with(Data::from(42_usize))
            .with(Data::from(-13_isize))
            .with(Data::from(2.4)),
    );
    println!("{}", table);
}
