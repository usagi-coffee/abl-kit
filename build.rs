use std::path::PathBuf;

fn main() {
    let dir: PathBuf = ["tree-sitter-abl", "src"].iter().collect();

    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .file(dir.join("scanner.c"))
        .warnings(false)
        .compile("tree-sitter-abl");
}
