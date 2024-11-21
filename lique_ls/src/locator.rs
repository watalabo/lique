use std::ops::Range;

use line_index::{LineIndex, TextSize};
use lsp_types::Position;

#[derive(Clone)]
pub struct Locator {
    index: LineIndex,
}

impl Locator {
    pub fn read_string(s: &str) -> Self {
        Self {
            index: LineIndex::new(s),
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
