#![feature(let_chains)]
pub mod lints;

#[cfg(test)]
pub(crate) mod tests {
    pub fn parse_python_source(source: &str) -> rustpython_parser::ast::Mod {
        rustpython_parser::parse(source, rustpython_parser::Mode::Module, "<test>").unwrap()
    }
}
