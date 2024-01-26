use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use encoding_rs::WINDOWS_1250;

mod defaults;
mod indent;
mod style;

pub fn fix_file(path: &String) -> Result<(), Box<dyn Error>> {
    let mut file;
    let mut is_win1250 = false;

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

    let mut output;
    output = indent::transform(&source);
    output = defaults::transform(&output);
    output = style::transform(&output);

    if is_win1250 {
        let (bytes, _, _) = WINDOWS_1250.encode(output.as_str());
        std::fs::write(path, bytes).expect("Failed to write to file");
    } else {
        std::fs::write(path, output).expect("Failed to write to file");
    }

    Ok(())
}
