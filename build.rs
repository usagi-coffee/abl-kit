use std::path::PathBuf;

fn main() {
    let abl_dir: PathBuf = ["tree-sitter-abl", "src"].iter().collect();
    let df_dir: PathBuf = ["tree-sitter-df", "src"].iter().collect();

    cc::Build::new()
        .include(&abl_dir)
        .file(abl_dir.join("parser.c"))
        .file(abl_dir.join("scanner.c"))
        .warnings(false)
        .compile("tree-sitter-abl");

    cc::Build::new()
        .include(&df_dir)
        .file(df_dir.join("parser.c"))
        .file(df_dir.join("scanner.c"))
        .warnings(false)
        .compile("tree-sitter-df");
}
