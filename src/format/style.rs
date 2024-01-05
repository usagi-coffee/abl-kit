use crate::parser;
use tree_sitter::TreeCursor;

const KEYWORDS: &'static str = include_str!("../../data/keywords.txt");

// Could be refactored into macro but whatever
fn keywords() -> Vec<&'static str> {
    KEYWORDS.lines().collect()
}

pub fn transform(source: &String) -> String {
    let mut parser = parser::setup();

    let mut output = source.clone();

    loop {
        let tree = parser
            .parse(&output, None)
            .expect("Failed to parse the file");
        let mut cursor = tree.walk();

        if traverse_tree(&mut cursor, &mut output) {
            break;
        }
    }

    output
}

fn traverse_tree(cursor: &mut TreeCursor, source: &mut String) -> bool {
    for node in cursor.node().children(cursor) {
        // Uppercase
        if keywords().contains(&node.kind()) {
            let range = node.range();
            source.replace_range(
                range.start_byte..range.end_byte,
                source[range.start_byte..range.end_byte]
                    .to_uppercase()
                    .as_str(),
            );

            // We don't need to rebuild tree as the change does not change the length of the output
        }

        traverse_tree(&mut node.walk(), source);
    }

    true
}
