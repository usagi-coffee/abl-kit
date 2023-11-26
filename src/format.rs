use crate::parser;

use tree_sitter::TreeCursor;
use std::path::Path;

struct State {
    indentation_level: usize,
    indentations: Vec<usize>,
}

const TAB_SPACES: usize = 2;

const INDENTATED_STATEMENTS: [&'static str; 19] = [
    "if_statement",
    "if_do_statement",
    "else_do_statement",
    "else_do_if_statement",
    "loop_statement",
    "do_statement",
    "repeat_statement",
    "function_statement",
    "procedure_statement",
    "transaction_statement",
    "for_statement",
    "find_statement",
    "case_statement",
    "assign_statement",
    "catch_statement",
    "finally_statement",
    "abl_statement",
    "do_block",
    "include"
];

const EXTENDED_STATEMENTS: [&'static str; 2] = ["else_do_statement", "else_do_if_statement"];

pub fn fix_file(file: &String) {
    let source: String;

    if file.starts_with("/") {
        source = std::fs::read_to_string(Path::new(file)).unwrap();
    } else {
        let wd = std::env::current_dir().unwrap();
        source = std::fs::read_to_string(wd.join(file).as_path()).unwrap();
    }

    let output = fix(&source);
    std::fs::write(file, output).expect("Failed to write to file");
}

pub fn fix(source: &String) -> String {
    let mut parser = parser::setup();

    let tree = parser.parse(&source, None).unwrap();

    let mut state = State {
        indentation_level: 0,
        indentations: Vec::new(),
    };

    // Fill all lines as zero indentation
    for _ in source.lines() {
        state.indentations.push(0);
    }

    let mut cursor = tree.walk();

    traverse_tree(&mut cursor, &mut state);

    let mut output = String::new();
    let mut was_empty = false;

    for (index, line) in source.lines().enumerate() {
        if was_empty && line.trim().is_empty() {
            continue;
        }

        output.push_str(&" ".repeat(state.indentations[index] * TAB_SPACES));
        output.push_str(line.trim());
        output.push_str("\n");

        was_empty = line.trim().is_empty();
    }

    if output.len() < 1 {
        panic!("Output is empty! Not writing to file");
    }

    output
}

fn traverse_tree(cursor: &mut TreeCursor, state: &mut State) {
    for node in cursor.node().children(cursor) {
        let start = node.start_position().row;
        let mut end = node.end_position().row;

        if INDENTATED_STATEMENTS.contains(&node.kind()) {
            if node.kind() == "abl_statement" && start == end {
                continue;
            }

            // Find terminator
            let mut cursor = node.walk();
            for node in node.children(&mut cursor) {
                if node.kind() == "END" {
                    end = node.end_position().row;
                }
            }

            let mut root = state.indentation_level.clone();
            if !EXTENDED_STATEMENTS.contains(&node.kind()) {
                state.indentation_level += 1;
            }

            for i in start + 1..end {
                state.indentations[i] = state.indentation_level;
            }

            traverse_tree(&mut node.walk(), state);

            if EXTENDED_STATEMENTS.contains(&node.kind()) {
                root -= 1;
            } else {
                state.indentation_level -= 1;
            }

            state.indentations[end] = root;
        } else {
            traverse_tree(&mut node.walk(), state);
        }
    }
}
