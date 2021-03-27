use crate::column::Column;
use crate::row::Row;
use std::fmt::{Display, Formatter, Result as FmtResult};

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
    if !self.columns.is_empty() && self.columns[0].len() != column.len() {
      panic!("Cannot add column with {} values to a matrix with {} rows", column.len(), self.columns[0].len());
    }
    self.columns.push(column);
  }

  pub fn add_row(&mut self, row: Row) {

    if row.len() != self.columns.len() {
      panic!("Cannot add row with {} values to a matrix with {} columns", row.len(), self.columns.len());
    }

    for (i, value) in row.into_iter().enumerate() {
      self.columns[i].add_value(value);
    }
  }

}

impl Display for Table {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {

    write!(f, "|")?;

    for column in self.columns.iter() {
      write!(f, " {} |", column.name())?;
    }

    if self.columns.is_empty() {
      writeln!(f, " <NO COLUMNS> |")?;
      return FmtResult::Ok(());
    } else {
      writeln!(f)?;
    }

    write!(f, "|")?;

    for i in 0..self.columns[0].len() {
      for column in self.columns.iter() {
        write!(f, " {} |", column[i])?;
      }
      writeln!(f)?;
    }

    FmtResult::Ok(())
  }
}