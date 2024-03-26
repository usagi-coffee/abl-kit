use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use encoding_rs::WINDOWS_1250;

mod extract;

pub fn universal_read(path: &String) -> Result<(String, bool), Box<dyn Error>> {
    let mut file;
    let mut is_win1250: bool = false;

    if path.starts_with("/") {
        file = File::open(Path::new(path))?;
    } else {
        let wd = std::env::current_dir().unwrap();
        file = File::open(wd.join(path).as_path())?;
    }

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let source = match String::from_utf8(buffer.clone()) {
        Ok(text) => text,
        Err(_) => {
            let (bytes, _, _) = WINDOWS_1250.decode(buffer.as_slice());
            is_win1250 = true;
            (*bytes).to_string()
        }
    };

    Ok((source, is_win1250))
}

pub fn strip_quotes(string: &str) -> String {
    string
        .chars()
        .skip(1)
        .take(string.len() - 2)
        .collect::<String>()
}

pub fn extract(path: &String, tag: String) -> Result<Option<String>, Box<dyn Error>> {
    let (source, _is_win1250) = universal_read(path)?;

    Ok(extract::extract_tag(&source, tag))
}
