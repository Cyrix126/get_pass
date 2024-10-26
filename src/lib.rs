use std::{error::Error, path::Path, process::Command, str::from_utf8};

pub fn get_password(path: &Path) -> Result<String, Box<dyn Error>> {
    let output = Command::new("pass").arg("show").arg(path).output()?;
    Ok(from_utf8(&output.stdout)?
        .trim_end_matches('\n')
        .to_string())
}

#[cfg(feature = "combine_with_url")]
pub mod url {

    use crate::{Error, Path};
    use url::Url;
    pub fn add_pass_to_url(url: &mut Url, path_pass: &Path) -> Result<(), Box<dyn Error>> {
        Ok(url
            .set_password(crate::get_password(path_pass).ok().as_deref())
            .map_err(|_| std::fmt::Error)?)
    }
}
