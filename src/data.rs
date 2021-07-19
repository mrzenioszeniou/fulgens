use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum Data {
    UInt { value: u64 },

    Int { value: i64 },

    Float { value: f64 },

    DateTime { value: DateTime<Utc> },
}

impl Data {
    pub fn same_type(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UInt { .. }, Self::UInt { .. })
            | (Self::Int { .. }, Self::Int { .. })
            | (Self::Float { .. }, Self::Float { .. })
            | (Self::DateTime { .. }, Self::DateTime { .. }) => true,
            _ => false,
        }
    }

    pub fn uint(&self) -> Option<&u64> {
        match self {
            Self::UInt { ref value } => Some(value),
            _ => None,
        }
    }

    pub fn int(&self) -> Option<&i64> {
        match self {
            Self::Int { ref value } => Some(value),
            _ => None,
        }
    }

    pub fn float(&self) -> Option<&f64> {
        match self {
            Self::Float { ref value } => Some(value),
            _ => None,
        }
    }

    pub fn datetime(&self) -> Option<&DateTime<Utc>> {
        match self {
            Self::DateTime { ref value } => Some(value),
            _ => None,
        }
    }
}

impl From<usize> for Data {
    fn from(from: usize) -> Self {
        Self::UInt { value: from as u64 }
    }
}

impl From<u32> for Data {
    fn from(from: u32) -> Self {
        Self::UInt { value: from as u64 }
    }
}

impl From<u64> for Data {
    fn from(from: u64) -> Self {
        Self::UInt { value: from }
    }
}

impl From<isize> for Data {
    fn from(from: isize) -> Self {
        Self::Int { value: from as i64 }
    }
}

impl From<i32> for Data {
    fn from(from: i32) -> Self {
        Self::Int { value: from as i64 }
    }
}

impl From<i64> for Data {
    fn from(from: i64) -> Self {
        Self::Int { value: from }
    }
}

impl From<f32> for Data {
    fn from(from: f32) -> Self {
        Self::Float { value: from as f64 }
    }
}

impl From<f64> for Data {
    fn from(from: f64) -> Self {
        Self::Float { value: from }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Int { value } => Display::fmt(value, f),
            Self::UInt { value } => Display::fmt(value, f),
            Self::Float { value } => Display::fmt(value, f),
            Self::DateTime { value } => Display::fmt(value, f),
        }
    }
}
