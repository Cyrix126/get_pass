use anyhow::Result;
use std::{path::Path, process::Command, str::from_utf8};

pub fn get_password(path: &Path) -> Result<String> {
    let output = Command::new("pass").arg("show").arg(path).output()?;
    Ok(from_utf8(&output.stdout)?
        .trim_end_matches('\n')
        .to_string())
}
