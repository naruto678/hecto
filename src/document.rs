use crate::Position;
use crate::Row;
use std::fs;
use std::io::Error;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    filename: Option<String>,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            filename: Some(filename.to_string()),
        })
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

    pub fn get_file_name(&self) -> String {
        if let Some(name) = self.filename.clone() {
            String::from(name)
        } else {
            "Default-Buffer".to_string()
        }
    }
    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else if at.y < self.len() {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
        }
    }
}
