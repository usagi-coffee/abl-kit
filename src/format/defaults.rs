use crate::parser;
use tree_sitter::{InputEdit, TreeCursor};

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
    for mut node in cursor.node().children(cursor) {
        // Add NO-UNDO
        if node.kind() == "variable_definition" {
            let mut has_no_undo = false;
            for child in node.named_children(&mut node.walk()) {
                if child.kind() == "variable_tuning" && child.child(0).unwrap().kind() == "NO-UNDO"
                {
                    has_no_undo = true;
                }
            }

            if !has_no_undo {
                let range = node.range();
                let type_node = node
                    .child_by_field_name("type")
                    .expect("Variable definition does not have type definition");

                source.insert_str(type_node.end_byte(), " NO-UNDO");

                node.edit(&InputEdit {
                    start_byte: range.start_byte,
                    old_end_byte: range.end_byte,
                    new_end_byte: range.end_byte + 8,
                    start_position: range.start_point,
                    old_end_position: range.end_point,
                    new_end_position: node.end_position(),
                });

                return false;
            }
        }

        traverse_tree(&mut node.walk(), source);
    }

    true
}
