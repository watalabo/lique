#![feature(let_chains)]

pub mod error;
pub mod lints;

use crate::error::Result;
use rustpython_parser::{
    ast::{Mod, ModModule},
    parse,
    source_code::SourceRange,
    Mode,
};
use std::{ops::Range, path::{Path, PathBuf}};

#[derive(Debug)]
pub struct SourceCode {
    pub code: String,
    path: PathBuf,
}

impl std::ops::Deref for SourceCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.code
    }
}

impl SourceCode {
    pub fn read_from_path<P: AsRef<Path>>(path: P) -> Self {
        let code = std::fs::read_to_string(&path).unwrap();
        let path = PathBuf::from(path.as_ref());
        Self { code, path }
    }

    pub fn parse(&self) -> crate::Result<ModModule> {
        let module = parse(&self.code, Mode::Module, self.path.to_str().unwrap())?;
        let Mod::Module(module) = module else {
            panic!("unreachable");
        };
        Ok(module)
    }
}

pub struct Diagnostic {
    pub message: String,
    pub range_zero_indexed: Range<usize>,
}

#[cfg(test)]
pub(crate) mod tests {
    pub fn parse_python_source(source: &str) -> rustpython_parser::ast::Mod {
        rustpython_parser::parse(source, rustpython_parser::Mode::Module, "<test>").unwrap()
    }
}
