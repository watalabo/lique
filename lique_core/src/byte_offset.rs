use core::ops::Range;
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
}

/// ByteOffsetLocator converts 0-indexed line numbers to a range of byte offsets in a content.
pub struct ByteOffsetLocator {
    /// i-th element is the byte offset of the start of the i-th line.
    line_offsets: Vec<usize>,
    pub contents_lines: Vec<String>,
}

impl ByteOffsetLocator {
    pub fn read_from_file(file_path: &str) -> Result<Self, ByteOffsetError> {
        let mut locator = ByteOffsetLocator {
            line_offsets: Vec::new(),
            contents_lines: Vec::new(),
        };
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        locator.read_contents(&mut reader)?;
        Ok(locator)
    }

    pub fn read_from_string(content: &str) -> Self {
        let mut locator = ByteOffsetLocator {
            line_offsets: Vec::new(),
            contents_lines: Vec::new(),
        };
        let mut reader = BufReader::new(content.as_bytes());
        locator.read_contents(&mut reader).unwrap();
        locator
    }

    fn read_contents<R: BufRead>(&mut self, reader: &mut R) -> Result<(), ByteOffsetError> {
        let mut offset = 0;

        let mut line = String::new();
        while reader.read_line(&mut line)? > 0 {
            self.contents_lines.push(line.clone());
            self.line_offsets.push(offset);
            offset += line.len();
            line.clear();
        }
        Ok(())
    }

    /// Locate a range of bytes that corresponds to the given 0-indexed line number.
    /// The range excludes leading whitespaces.
    pub fn locate_line(&self, line_number: usize) -> Result<Range<usize>, ByteOffsetError> {
        if line_number >= self.line_offsets.len() {
            return Err(ByteOffsetError::InvalidLineNumber(line_number));
        }
        let line_start_offset = self.line_offsets[line_number];
        let line = &self.contents_lines[line_number];
        let line_end_offset = line_start_offset + line.len();
        // Skip leading whitespaces
        let line_start_offset = line_start_offset
            + line
                .chars()
                // But don't skip '\n', which is empty line
                .take_while(|&c| c.is_whitespace() && c != '\n')
                .count();
        Ok(line_start_offset..line_end_offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_case() {
        let content = "Hello\nWorld\nThis is a test.";
        let locator = ByteOffsetLocator::read_from_string(content);
        // "World"
        assert_eq!(locator.locate_line(1).unwrap(), 6..12);
    }

    #[test]
    fn test_one_line() {
        let content = "Hello World";
        let locator = ByteOffsetLocator::read_from_string(content);
        // "Hello World"
        assert_eq!(locator.locate_line(0).unwrap(), 0..11);
    }

    #[test]
    fn test_including_empty_line() {
        let content = "Hello\n\nWorld";
        let locator = ByteOffsetLocator::read_from_string(content);
        // empty line
        assert_eq!(locator.locate_line(1).unwrap(), 6..7);
        // "World"
        assert_eq!(locator.locate_line(2).unwrap(), 7..12);
    }

    #[test]
    fn test_including_leading_whitespaces() {
        let content = "Hello\n  World";
        let locator = ByteOffsetLocator::read_from_string(content);
        // "  World"
        assert_eq!(locator.locate_line(1).unwrap(), 8..13);
    }

    #[test]
    fn test_totally_empty_content() {
        let content = "";
        let locator = ByteOffsetLocator::read_from_string(content);
        assert!(matches!(
            locator.locate_line(0),
            Err(ByteOffsetError::InvalidLineNumber(0))
        ));
    }

    #[test]
    fn test_invalid_line_number() {
        let content = "Hello\nWorld";
        let locator = ByteOffsetLocator::read_from_string(content);
        assert!(matches!(
            locator.locate_line(2),
            Err(ByteOffsetError::InvalidLineNumber(2))
        ));
    }
}
