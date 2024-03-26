use crate::parser;
use tree_sitter::TreeCursor;

pub fn extract_tag(source: &String, tag: String) -> Option<String> {
    let mut parser = parser::setup_abl();
    let mut output = source.clone();

    let tree = parser
        .parse(&output, None)
        .expect("Failed to parse the file");
    let mut cursor = tree.walk();

    traverse_tree(&mut cursor, &mut output, tag)
}

fn traverse_tree(cursor: &mut TreeCursor, source: &mut String, tag: String) -> Option<String> {
    for node in cursor.node().children(cursor) {
        if node.is_error() {
            continue;
        }

        if node.kind() == "comment" {
            let range = node.range();
            let text = &source[range.start_byte..range.end_byte];

            // Ensure tag is found
            if let Some(tag_index) = text.find(&format!("@{}", tag)) {
                let start_index = text.find('{')?;
                let end_index = text.find('}')?;

                if tag_index < start_index && end_index > start_index {
                    return Some(text[start_index + 1..end_index].to_string());
                }
            }
        }
    }

    None
}
