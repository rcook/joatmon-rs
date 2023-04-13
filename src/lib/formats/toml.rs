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
pub struct TomlError {
    path: PathBuf,
    message: String,
    span: Option<std::ops::Range<usize>>,
}

impl std::fmt::Display for TomlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.span {
            Some(s) => write!(
                f,
                "{} at span {}:{} in {}",
                self.message,
                s.start,
                s.end,
                self.path.display()
            ),
            None => write!(f, "{} in {}", self.message, self.path.display()),
        }
    }
}

impl std::error::Error for TomlError {}

pub fn read_toml_file<T, P>(path: P) -> Result<T>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let s = read_text_file(path.as_ref())?;
    let value = toml::from_str::<T>(&s).map_err(|e| translate_toml_error(e, &path))?;
    Ok(value)
}

fn translate_toml_error<P>(e: toml::de::Error, path: P) -> Box<dyn std::error::Error>
where
    P: AsRef<Path>,
{
    Box::new(TomlError {
        path: PathBuf::from(path.as_ref()),
        message: String::from(e.message()),
        span: e.span(),
    })
}
