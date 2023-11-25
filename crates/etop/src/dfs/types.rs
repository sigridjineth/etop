use crate::EtopError;
use cryo_freeze::Datatype;
use polars::prelude::*;
use std::collections::HashMap;
use toolstr::{BinaryFormat, NumberFormat, StringFormat};

pub trait Dataset {
    /// which datasets the view is constructed from
    fn inputs() -> Vec<Datatype>;

    /// transform inputs into the data needed for a view
    fn transform(&self, dfs: HashMap<Datatype, DataFrame>) -> Result<DataFrame, EtopError>;

    /// default columns
    fn default_columns(&self) -> Vec<String>;

    /// default format for each column
    fn default_column_formats(&self) -> HashMap<String, ColumnFormat>;
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ColumnFormat {
    pub name: String,
    pub display_name: String,
    pub min_width: Option<usize>,
    pub max_width: Option<usize>,
    pub format: Option<Format>,
}

#[derive(Debug, Clone)]
pub enum Format {
    Number(NumberFormat),
    Binary(BinaryFormat),
    String(StringFormat),
}

impl Default for ColumnFormat {
    fn default() -> ColumnFormat {
        ColumnFormat {
            name: "".to_string(),
            display_name: "".to_string(),
            min_width: None,
            max_width: None,
            format: None,
        }
    }
}

// extract formats
impl ColumnFormat {
    pub fn number_format(&self) -> Result<NumberFormat, EtopError> {
        match self.format.as_ref() {
            Some(Format::Number(format)) => Ok(format.clone()),
            None => {
                let mut fmt = NumberFormat::new();
                if let Some(min_width) = self.min_width {
                    fmt.min_width = min_width
                };
                if let Some(max_width) = self.max_width {
                    fmt.max_width = max_width
                };
                Ok(fmt)
            },
            _ => {
                let msg = format!("column {} requires NumberFormat", self.name);
                Err(EtopError::MismatchedFormatType(msg))
            }
        }
    }

    pub fn binary_format(&self) -> Result<BinaryFormat, EtopError> {
        match self.format.as_ref() {
            Some(Format::Binary(format)) => Ok(format.clone()),
            None => {
                let mut fmt = BinaryFormat::new();
                if let Some(min_width) = self.min_width {
                    fmt.min_width = min_width
                };
                if let Some(max_width) = self.max_width {
                    fmt.max_width = max_width
                };
                Ok(fmt)
            },
            _ => {
                let msg = format!("column {} requires NumberFormat", self.name);
                Err(EtopError::MismatchedFormatType(msg))
            }
        }
    }
}

// builder
impl ColumnFormat {
    pub fn new() -> ColumnFormat {
        ColumnFormat::default()
    }

    pub fn name<T: AsRef<str>>(mut self, name: T) -> ColumnFormat {
        let name = name.as_ref().to_string();
        self.name = name.clone();
        if self.display_name.is_empty() {
            self.display_name = name
        };
        self
    }

    pub fn display_name<T: AsRef<str>>(mut self, display_name: T) -> ColumnFormat {
        self.display_name = display_name.as_ref().to_string();
        self
    }

    pub fn newline_underscores(mut self) -> ColumnFormat {
        self.display_name = self.display_name.replace('_', "\n");
        self
    }

    pub fn min_width(mut self, min_width: usize) -> ColumnFormat {
        self.min_width = Some(min_width);
        self
    }

    pub fn no_min_width(mut self) -> ColumnFormat {
        self.min_width = None;
        self
    }

    pub fn max_width(mut self, max_width: usize) -> ColumnFormat {
        self.max_width = Some(max_width);
        self
    }

    pub fn no_max_width(mut self) -> ColumnFormat {
        self.max_width = None;
        self
    }
}
