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
use crate::result::Result;
use std::fs::{create_dir_all, read_to_string, write, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum DirectoryError {
    AlreadyExists { path: PathBuf },
    NotFound { path: PathBuf },
}

impl std::fmt::Display for DirectoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExists { path } => {
                write!(f, "Directory {} already exists", path.display())
            }
            Self::NotFound { path } => write!(f, "Directory {} not found", path.display()),
        }
    }
}

impl std::error::Error for DirectoryError {}

#[derive(Debug)]
pub enum FileError {
    AlreadyExists { path: PathBuf },
    NotFound { path: PathBuf },
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExists { path } => write!(f, "File {} already exists", path.display()),
            Self::NotFound { path } => write!(f, "File {} not found", path.display()),
        }
    }
}

impl std::error::Error for FileError {}

pub fn read_text_file<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    Ok(read_to_string(path.as_ref())?)
}

pub fn safe_create_file<P>(path: P, overwrite: bool) -> Result<File>
where
    P: AsRef<Path>,
{
    ensure_dir(&path)?;

    let mut options = OpenOptions::new();
    options.write(true);
    if overwrite {
        options.create(true);
    } else {
        options.create_new(true);
    }

    options
        .open(&path)
        .map_err(|e| translate_file_error(e, &path))
}

pub fn safe_write_file<P, C>(path: P, contents: C, overwrite: bool) -> Result<()>
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    ensure_dir(&path)?;

    if overwrite {
        write(&path, contents).map_err(|e| translate_file_error(e, &path))?;
    } else {
        let mut file = safe_create_file(&path, overwrite)?;
        file.write_all(contents.as_ref())
            .map_err(|e| translate_file_error(e, &path))?;
    }

    Ok(())
}

fn translate_dir_error<P>(e: std::io::Error, path: P) -> Box<dyn std::error::Error>
where
    P: AsRef<Path>,
{
    use std::io::ErrorKind::*;

    match e.kind() {
        AlreadyExists => Box::new(DirectoryError::AlreadyExists {
            path: PathBuf::from(path.as_ref()),
        }),
        NotFound => Box::new(DirectoryError::NotFound {
            path: PathBuf::from(path.as_ref()),
        }),
        _ => e.into(),
    }
}

fn translate_file_error<P>(e: std::io::Error, path: P) -> Box<dyn std::error::Error>
where
    P: AsRef<Path>,
{
    use std::io::ErrorKind::*;

    match e.kind() {
        AlreadyExists => Box::new(FileError::AlreadyExists {
            path: PathBuf::from(path.as_ref()),
        }),
        NotFound => Box::new(FileError::NotFound {
            path: PathBuf::from(path.as_ref()),
        }),
        _ => e.into(),
    }
}

fn ensure_dir<P>(file_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut dir = PathBuf::new();
    dir.push(&file_path);
    dir.pop();
    create_dir_all(&dir).map_err(|e| translate_dir_error(e, &dir))?;
    Ok(())
}
