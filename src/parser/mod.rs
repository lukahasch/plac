use crate::error::Error;

use std::cmp::Ordering;

/// multi file capabilities later
pub struct Parser {
    pub file_name: String,
    pub input: String,
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
pub struct Position {
    pub line: (usize, usize),
    pub column: (usize, usize),
    pub file: String,
}

impl Position {
    pub fn is_multi_line(&self) -> bool {
        self.line.0 != self.line.1
    }

    pub fn contains(&self, position: &Position) -> bool {
        self.line.0 <= position.line.0 && self.line.1 >= position.line.1 && self.column.0 <= position.column.0 && self.column.1 >= position.column.1
    }

    pub fn expand_to_line(&self) -> Position {
        Position {
            line: (self.line.0, self.line.1),
            column: (1, 10000),
            file: self.file.clone(),
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.file != other.file {
            return None;
        }
        match (self.line.0.cmp(&other.line.0), self.column.0.cmp(&other.column.0)) {
            (Ordering::Equal, i) => Some(i),
            (i, _) => Some(i),
        }
    }
}

impl From<Position> for String {
    fn from(position: Position) -> String {
        if position.line.0 == position.line.1 {
            format!("{}:{}:{}-{}",position.file, position.line.0, position.column.0, position.column.1)
        } else {
            format!("{}:{}:{}-{}:{}",position.file, position.line.0, position.column.0, position.line.1, position.column.1)
        }
    }
}

impl Parser {
    pub fn new(file_name: String, input: String) -> Self {
        Self {
            file_name,
            input,
            index: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn aquire(&self, position: &Position) -> String {
        let mut result = String::new();
        let mut in_str = false;
        let mut column = 1;
        let mut line = 1;
        for c in self.input.chars() {
            match (c, in_str, position.contains(&Position { file: self.file_name.clone(), line: (line, line), column: (column, column) })) {
                ('"', _, true) => {
                    in_str = !in_str;
                    result.push(c);
                }
                ('"', _, false) => {
                    in_str = !in_str;
                }
                (_, true, true) => result.push(c),
                ('\n', false, true) => {
                    line += 1;
                    column = 0;
                    result.push(c);
                }
                ('\n', false, false) => {
                    line += 1;
                    column = 0;
                }
                (_, false, true) => result.push(c),
                (_, _, false) => (),
            }
            column += 1;
        }
        result
    }
}

pub trait Parsable where Self: Sized {
    fn parse(&mut self,parser: &mut Parser) -> Result<Self, Error>;
}
