use crate::Position;
use crate::Row;
use std::{fs, io::{Error, Write}};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    //essentially "passed down" from Editor
    pub file_name: String,
    changed : bool,
}

impl Document {
    pub fn new(file_name : &str) -> Document {
        Document {rows : Vec::new(), file_name : file_name.to_string(), changed : false}
    }
    pub fn open(file_name: &String) -> Result<Self, std::io::Error> {
        let file_as_string = fs::read_to_string(file_name)?;
        let mut rows = Vec::new();
        for value in file_as_string.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            file_name: file_name.to_string(),
            changed : false,
        })
    }

    pub fn save(&mut self) ->  Result<(), Error> {
        let mut file = fs::File::create(&self.file_name)?;
        for row in &self.rows {
            file.write_all(&row.text.as_bytes())?;
            file.write_all(b"\n")?;
        }
        self.changed = false;
        Ok(())
    }
    fn insert_newline(&mut self, position: &Position) -> Result<(), Error> {
        if position.y > self.len() {
            return Ok(());
        }

        if position.y == self.len() {
            self.rows.push(Row::default());
            return Ok(());
        }

        let new_row = self
            .rows
            .get_mut(position.y)
            .ok_or(Error::new(
                std::io::ErrorKind::InvalidInput,
                "There was an error in inserting",
            ))?
            .split_row(position.x);
        self.rows.insert(position.y + 1, new_row);
        Ok(())
    }

    pub fn insert(&mut self, position: &Position, c: char) -> Result<(), Error> {
        if position.y > self.len() {
            return Ok(());
        }
        self.changed = true;

        if c == '\n' {
            self.insert_newline(position)?;
            return Ok(());
        }
        if position.y == self.len() {
            let mut new_row = Row::default();
            new_row.insert(c, 0);
            self.rows.push(new_row);
        } else {
            //need mutable not immutable reference so direct access
            let current_row = self.rows.get_mut(position.y).ok_or(Error::new(
                std::io::ErrorKind::InvalidInput,
                "There was an error in inserting",
            ))?;
            current_row.insert(c, position.x);
        }
        Ok(())
    }
    pub fn delete(&mut self, position: &Position) -> Result<(), Error> {
        if position.y >= self.len() {
            return Ok(());
        }
        //need mutable not immutable reference so direct access
        self.changed = true;
        if position.x
            == self
                .rows
                .get_mut(position.y)
                .ok_or(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "There was an error in deleting",
                ))?
                .len()
            && position.y + 1 < self.len()
        {
            let next_row = self.rows.remove(position.y + 1);
            let current_row = self.rows.get_mut(position.y).ok_or(Error::new(
                std::io::ErrorKind::InvalidInput,
                "There was an error in deleting",
            ))?;
            current_row.append(&next_row);
        } else {
            let current_row = self.rows.get_mut(position.y).ok_or(Error::new(
                std::io::ErrorKind::InvalidInput,
                "There was an error in deleting",
            ))?;
            current_row.delete(position.x);
        }
        Ok(())
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
    pub fn is_changed(&self) -> bool {
        self.changed
    }
}
