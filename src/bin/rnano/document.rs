use crate::Row;
use std::fs;

#[derive(Default)]
pub struct Document {
    rows : Vec<Row>,
    //essentially "passed down" from Editor
    pub file_name : Option<String>,
}

impl Document {
    pub fn open(file_name : &String) -> Result<Self, std::io::Error> {
        let file_as_string = fs::read_to_string(file_name)?;
        let mut rows = Vec::new();
        for value in file_as_string.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self { 
            rows,
            file_name : Some(file_name.to_string())
         })
    }

    pub fn row(&self, index : usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}