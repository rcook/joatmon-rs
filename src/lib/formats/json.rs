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
pub struct JsonError {
    _kind: JsonErrorKind,
    path: PathBuf,
    message: String,
    _line: usize,
    _column: usize,
}

#[derive(Debug)]
pub enum JsonErrorKind {
    Data,
    Eof,
    Io,
    Syntax,
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} in {}", self.message, self.path.display())
    }
}

impl std::error::Error for JsonError {}

pub fn read_json_file<T, P>(path: P) -> Result<T>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let s = read_text_file(path.as_ref())?;
    let value = serde_json::from_str::<T>(&s).map_err(|e| translate_json_error(e, &path))?;
    Ok(value)
}

fn translate_json_error<P>(e: serde_json::Error, path: P) -> Box<dyn std::error::Error>
where
    P: AsRef<Path>,
{
    use serde_json::error::Category::*;

    let kind = match e.classify() {
        Data => JsonErrorKind::Data,
        Eof => JsonErrorKind::Eof,
        Io => JsonErrorKind::Io,
        Syntax => JsonErrorKind::Syntax,
    };
    Box::new(JsonError {
        _kind: kind,
        path: PathBuf::from(path.as_ref()),
        message: e.to_string(),
        _line: e.line(),
        _column: e.column(),
    })
}
