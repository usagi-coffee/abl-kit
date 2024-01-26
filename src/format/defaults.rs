use crate::parser;
use tree_sitter::TreeCursor;

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
        if node.is_error() {
            continue;
        }

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
                let mut target_node;

                target_node = node.child_by_field_name("type");
                if target_node.is_none() {
                    target_node = node.child_by_field_name("like");
                }

                if target_node.is_none() {
                    panic!("Variable definition does not have type definition");
                }

                let target_node = target_node.unwrap();

                source.insert_str(target_node.end_byte(), " NO-UNDO");
                return false;
            }
        }

        if !traverse_tree(&mut node.walk(), source) {
            return false;
        }
    }

    true
}
