use tree_sitter::{Language, Parser, TreeCursor};

use std::path::Path;

extern "C" {
    fn tree_sitter_abl() -> Language;
}

pub fn fix(file: &String) {
    if file.starts_with("/") {
        fix_file(Path::new(file));
        return;
    }

    let wd = std::env::current_dir().unwrap();
    fix_file(wd.join(file).as_path());
}

struct State {
    indentation_level: usize,
    indentations: Vec<usize>,
}

const TAB_SPACES: usize = 2;

const INDENTATED_STATEMENTS: [&'static str; 13] = [
    "if_statement",
    "if_do_statement",
    "else_do_statement",
    "else_do_if_statement",
    "loop_statement",
    "function_statement",
    "procedure_statement",
    "transaction_statement",
    "for_statement",
    "find_statement",
    "case_statement",
    "assign_statement",
    "abl_statement",
];

const EXTENDED_STATEMENTS: [&'static str; 2] = ["else_do_statement", "else_do_if_statement"];

pub fn fix_file(file: &Path) {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_abl() };
    parser.set_language(language).unwrap();

    let source_code = std::fs::read_to_string(file).unwrap();
    let tree = parser.parse(&source_code, None).unwrap();

    let mut state = State {
        indentation_level: 0,
        indentations: Vec::new(),
    };

    // Fill all lines as zero indentation
    for _ in source_code.lines() {
        state.indentations.push(0);
    }

    let mut cursor = tree.walk();

    traverse_tree(&mut cursor, &mut state);

    let mut output = String::new();

    for (index, line) in source_code.lines().enumerate() {
        output.push_str(&" ".repeat(state.indentations[index] * TAB_SPACES));
        output.push_str(line.trim());
        output.push_str("\n");
    }

    if output.len() < 1 {
        panic!("Output is empty! Not writing to file");
    }

    std::fs::write(file, output).expect("Failed to write to file");
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

            println!(
                "{} {:?} {:?}",
                node.kind(),
                node.start_position(),
                node.end_position()
            );

            let mut root = state.indentation_level.clone();
            if !EXTENDED_STATEMENTS.contains(&node.kind()) {
                state.indentation_level += 1;
            }

            println!(
                "indentating {} from {} to {}",
                state.indentation_level,
                start + 1,
                end + 1
            );

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
