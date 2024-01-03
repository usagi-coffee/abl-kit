use crate::parser;
use tree_sitter::{InputEdit, TreeCursor};

const KEYWORDS: &'static str = include_str!("../../data/keywords.txt");

// Could be refactored into macro but whatever
fn keywords() -> Vec<&'static str> {
    KEYWORDS.lines().collect()
}

pub fn transform(source: &String) -> String {
    let mut parser = parser::setup();

    let mut output = source.clone();

    let tree = parser.parse(&output, None).unwrap();
    let mut cursor = tree.walk();

    traverse_tree(&mut cursor, &mut output);
    output
}

fn traverse_tree(cursor: &mut TreeCursor, source: &mut String) {
    for mut node in cursor.node().children(cursor) {
        if keywords().contains(&node.kind()) {
            let range = node.range();

            source.replace_range(
                range.start_byte..range.end_byte,
                source[range.start_byte..range.end_byte]
                    .to_uppercase()
                    .as_str(),
            );

            let edit = InputEdit {
                start_byte: range.start_byte,
                old_end_byte: range.end_byte,
                new_end_byte: range.start_byte + &node.kind().len() + 4,
                start_position: range.start_point,
                old_end_position: range.end_point,
                new_end_position: node.end_position(),
            };

            node.edit(&edit);
        }

        traverse_tree(&mut node.walk(), source);
    }
}

