use std::{ops::Range, path::Path};

use line_index::{LineIndex, TextSize};
use lsp_types::Position;
use std::fs;

#[derive(Clone)]
pub struct Locator {
    index: LineIndex,
}

impl Locator {
    pub fn read_file<P: AsRef<Path>>(path: P) -> Self {
        let text = fs::read_to_string(path).expect("Unable to read file");
        Self {
            index: LineIndex::new(&text),
        }
    }

    pub fn locate(&self, range: Range<usize>) -> lsp_types::Range {
        let start = TextSize::new(range.start as u32);
        let start = self.index.line_col(start);
        let end = TextSize::new(range.end as u32);
        let end = self.index.line_col(end);
        lsp_types::Range {
            start: Position {
                line: start.line,
                character: start.col,
            },
            end: Position {
                line: end.line,
                character: end.col,
            },
        }
    }
}
