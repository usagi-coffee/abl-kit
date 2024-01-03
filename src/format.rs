use std::path::Path;

mod indent;
mod style;

pub fn fix_file(file: &String) {
    let source: String;

    if file.starts_with("/") {
        source = std::fs::read_to_string(Path::new(file)).unwrap();
    } else {
        let wd = std::env::current_dir().unwrap();
        source = std::fs::read_to_string(wd.join(file).as_path()).unwrap();
    }

    let mut output;
    output = indent::transform(&source);
    output = style::transform(&output);

    std::fs::write(file, output).expect("Failed to write to file");
}
