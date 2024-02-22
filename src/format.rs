use crate::utils;
use encoding_rs::WINDOWS_1250;
use std::error::Error;

mod defaults;
mod indent;
mod style;

pub fn format_file(path: &String) -> Result<(), Box<dyn Error>> {
    let (source, is_win1250) = utils::universal_read(path)?;

    let mut output;
    output = indent::transform(&source);
    output = style::transform(&output);

    if is_win1250 {
        let (bytes, _, _) = WINDOWS_1250.encode(output.as_str());
        std::fs::write(path, bytes).expect("Failed to write to file");
    } else {
        std::fs::write(path, output).expect("Failed to write to file");
    }

    Ok(())
}

pub fn fix_file(path: &String) -> Result<(), Box<dyn Error>> {
    let (source, is_win1250) = utils::universal_read(path)?;

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
