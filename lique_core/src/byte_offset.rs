use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ByteOffsetError {
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("Invalid line number")]
    InvalidLineNumber(usize),
    #[error("Invalid column number")]
    InvalidColumnNumber(usize),
}

/// ByteOffsetLocator converts 0-indexed line and column numbers to byte offsets in a content.
pub struct ByteOffsetLocator {
    /// i-th element is the byte offset of the start of the i-th line.
    line_offsets: Vec<usize>,
    contents: String,
}

impl ByteOffsetLocator {
    pub fn read_from_file(file_path: &str) -> Result<Self, ByteOffsetError> {
        let mut locator = ByteOffsetLocator {
            line_offsets: Vec::new(),
            contents: String::new(),
        };
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        locator.read_contents(&mut reader)?;
        Ok(locator)
    }

    pub fn read_from_string(content: &str) -> Self {
        let mut locator = ByteOffsetLocator {
            line_offsets: Vec::new(),
            contents: String::new(),
        };
        let mut reader = BufReader::new(content.as_bytes());
        locator.read_contents(&mut reader).unwrap();
        locator
    }

    fn read_contents<R: BufRead>(&mut self, reader: &mut R) -> Result<(), ByteOffsetError> {
        let mut offset = 0;

        let mut line = String::new();
        while reader.read_line(&mut line)? > 0 {
            self.contents.push_str(&line);
            self.line_offsets.push(offset);
            offset += line.as_bytes().len();
            line.clear();
        }
        Ok(())
    }

    pub fn locate(
        &self,
        line_number: usize,
        column_number: usize,
    ) -> Result<usize, ByteOffsetError> {
        if line_number >= self.line_offsets.len() {
            return Err(ByteOffsetError::InvalidLineNumber(line_number));
        }
        if column_number
            > self.contents[self.line_offsets[line_number]..]
                .lines()
                .next()
                .unwrap_or("")
                .len()
        {
            return Err(ByteOffsetError::InvalidColumnNumber(column_number));
        }

        let line_start_offset = self.line_offsets[line_number];
        Ok(line_start_offset + column_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_case() {
        let content = "Hello\nWorld\nThis is a test.";
        let locator = ByteOffsetLocator::read_from_string(content);
        assert_eq!(locator.locate(1, 3).unwrap(), 9); // "l" in "World"
    }

    #[test]
    fn test_one_line() {
        let content = "Hello World";
        let locator = ByteOffsetLocator::read_from_string(content);
        assert_eq!(locator.locate(0, 4).unwrap(), 4); // "o" in "Hello"
    }

    #[test]
    fn test_including_empty_line() {
        let content = "Hello\n\nWorld";
        let locator = ByteOffsetLocator::read_from_string(content);
        assert_eq!(locator.locate(1, 0).unwrap(), 6); // start of the empty line
        assert_eq!(locator.locate(2, 1).unwrap(), 8); // "o" in "World"
    }

    #[test]
    fn test_end_of_line() {
        let content = "Hello\nWorld";
        let locator = ByteOffsetLocator::read_from_string(content);
        assert_eq!(locator.locate(0, 5).unwrap(), 5); // end of "Hello"
    }

    #[test]
    fn test_totally_empty_content() {
        let content = "";
        let locator = ByteOffsetLocator::read_from_string(content);
        assert!(matches!(
            locator.locate(1, 0),
            Err(ByteOffsetError::InvalidLineNumber(1))
        ));
    }

    #[test]
    fn test_invalid_line_number() {
        let content = "Hello\nWorld";
        let locator = ByteOffsetLocator::read_from_string(content);
        assert!(matches!(
            locator.locate(2, 0),
            Err(ByteOffsetError::InvalidLineNumber(2))
        ));
    }

    #[test]
    fn test_invalid_column_number() {
        let content = "Hello\nWorld";
        let locator = ByteOffsetLocator::read_from_string(content);
        assert!(matches!(
            locator.locate(1, 10),
            Err(ByteOffsetError::InvalidColumnNumber(10))
        ));
    }
}
