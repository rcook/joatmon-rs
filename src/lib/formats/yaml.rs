// Copyright (c) 2023 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use crate::fs::read_text_file;
use crate::result::Result;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct YamlError {
    path: PathBuf,
    message: String,
    location: Option<serde_yaml::Location>,
}

impl std::fmt::Display for YamlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.location {
            Some(loc) => write!(
                f,
                "{} at line {} column {} in {}",
                self.message,
                loc.line(),
                loc.column(),
                self.path.display()
            ),
            None => write!(f, "{} in {}", self.message, self.path.display()),
        }
    }
}

impl std::error::Error for YamlError {}

pub fn read_yaml_file<T, P>(path: P) -> Result<T>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let s = read_text_file(path.as_ref())?;
    let value = serde_yaml::from_str::<T>(&s).map_err(|e| translate_yaml_error(e, &path))?;
    Ok(value)
}

fn translate_yaml_error<P>(e: serde_yaml::Error, path: P) -> Box<dyn std::error::Error>
where
    P: AsRef<Path>,
{
    Box::new(YamlError {
        path: PathBuf::from(path.as_ref()),
        message: e.to_string(),
        location: e.location(),
    })
}
