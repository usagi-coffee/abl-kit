use tree_sitter::{Language, Parser};

extern "C" {
    fn tree_sitter_abl() -> Language;
}

pub fn setup() -> Parser {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_abl() };
    parser.set_language(language).unwrap();

    parser
}

