use tree_sitter::{Language, Parser};

extern "C" {
    fn tree_sitter_abl() -> Language;
    fn tree_sitter_df() -> Language;
}

pub fn setup_abl() -> Parser {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_abl() };
    parser.set_language(language).unwrap();

    parser
}

pub fn setup_df() -> Parser {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_df() };
    parser.set_language(language).unwrap();

    parser
}
