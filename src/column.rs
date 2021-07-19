use crate::data::Data;
use std::ops::Index;

pub struct Column {
    name: String,
    values: Vec<Data>,
}

impl Column {
    pub fn new(name: &str) -> Self {
        Column {
            name: String::from(name),
            values: vec![],
        }
    }

    pub fn add_value(&mut self, value: Data) {
        if self
            .values
            .get(0)
            .map(|v| v.same_type(&value))
            .unwrap_or(true)
        {
            self.values.push(value);
        } else {
            panic!("Can't add field to column '{}'", self.name);
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl Index<usize> for Column {
    type Output = Data;
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}
