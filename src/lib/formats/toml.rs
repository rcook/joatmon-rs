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
use anyhow::Error as AnyhowError;
use serde::de::DeserializeOwned;
use std::error::Error as StdError;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;
use thiserror::Error;
use toml::de::Error as TomlDeError;
use toml_edit::{Document, TomlError as TomlEditError};

#[allow(unused)]
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum TomlErrorKind {
    Syntax,
    Other,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct TomlError(#[from] TomlErrorImpl);

impl TomlError {
    #[allow(unused)]
    pub fn kind(&self) -> TomlErrorKind {
        match self.0 {
            TomlErrorImpl::Syntax { .. } => TomlErrorKind::Syntax,
            _ => TomlErrorKind::Other,
        }
    }

    #[allow(unused)]
    pub fn is_syntax(&self) -> bool {
        self.kind() == TomlErrorKind::Syntax
    }

    #[allow(unused)]
    pub fn is_other(&self) -> bool {
        self.kind() == TomlErrorKind::Other
    }

    fn other<E>(e: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self(TomlErrorImpl::Other(AnyhowError::new(e)))
    }

    fn convert<P>(e: TomlDeError, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let message = match e.span() {
            Some(s) => format!(
                "{} at span {}:{} in {}",
                e.message(),
                s.start,
                s.end,
                path.as_ref().display()
            ),
            None => format!("{} in {}", e.message(), path.as_ref().display()),
        };

        Self(TomlErrorImpl::Syntax {
            message,
            path: path.as_ref().to_path_buf(),
            span: e.span(),
        })
    }

    fn convert_edit<P>(e: TomlEditError, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let message = match e.span() {
            Some(s) => format!(
                "{} at span {}:{} in {}",
                e.message(),
                s.start,
                s.end,
                path.as_ref().display()
            ),
            None => format!("{} in {}", e.message(), path.as_ref().display()),
        };

        Self(TomlErrorImpl::Syntax {
            message,
            path: path.as_ref().to_path_buf(),
            span: e.span(),
        })
    }
}

#[derive(Debug, Error)]
enum TomlErrorImpl {
    #[error("{message}")]
    Syntax {
        message: String,
        path: PathBuf,
        span: Option<std::ops::Range<usize>>,
    },
    #[error(transparent)]
    Other(AnyhowError),
}

#[allow(unused)]
pub fn read_toml_file<T, P>(path: P) -> StdResult<T, TomlError>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let s = read_text_file(path.as_ref()).map_err(TomlError::other)?;
    let value = toml::from_str::<T>(&s).map_err(|e| TomlError::convert(e, &path))?;
    Ok(value)
}

#[allow(unused)]
pub fn read_toml_file_edit<P>(path: P) -> StdResult<Document, TomlError>
where
    P: AsRef<Path>,
{
    let s = read_text_file(path.as_ref()).map_err(TomlError::other)?;
    let doc = s
        .parse::<Document>()
        .map_err(|e| TomlError::convert_edit(e, &path))?;
    Ok(doc)
}

#[cfg(test)]
mod tests {
    use super::{read_toml_file, read_toml_file_edit, TomlErrorKind};
    use anyhow::Result;
    use std::fs::write;
    use tempdir::TempDir;
    use toml::{toml, Value};

    #[test]
    fn test_read_toml_file_succeeds() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("swiss-army-knife-test")?;
        let path = temp_dir.path().join("file.toml");
        write(&path, "message = \"hello-world\"")?;

        // Act
        let value = read_toml_file::<toml::Table, _>(&path)?;

        // Assert
        assert_eq!(toml!(message = "hello-world"), value);
        Ok(())
    }

    #[test]
    fn test_read_toml_file_invalid_fails() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("swiss-army-knife-test")?;
        let path = temp_dir.path().join("file.toml");
        write(&path, "xxx{\"message\": \"hello-world\"}")?;

        // Act
        let e = match read_toml_file::<Value, _>(&path) {
            Ok(_) => panic!("read_toml_file must fail"),
            Err(e) => e,
        };

        // Assert
        assert_eq!(TomlErrorKind::Syntax, e.kind());
        assert!(e.is_syntax());
        assert!(!e.is_other());
        let message = format!("{}", e);
        assert!(message.contains(path.to_str().expect("must be valid string")));
        Ok(())
    }

    #[test]
    fn test_read_toml_file_edit_succeeds() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("swiss-army-knife-test")?;
        let path = temp_dir.path().join("file.toml");
        let cargo_toml = r#"[package]
name = "swiss-army-knife"
version = "0.0.0"
edition = "2021"

[lib]
name = "swiss_army_knife"
path = "src/lib/mod.rs"

[[bin]]
name = "swiss-army-knife"
path = "src/bin/main.rs"

[dependencies]
anyhow = "1.0.70"
colored = "2.0.0"
serde = { version = "1.0.160", features = ["derive"] }
serde_json = "1.0.96"
serde_yaml = "0.9.21"
tempdir = "0.3.7"
thiserror = "1.0.40"
toml = "0.7.3"
toml_edit = "0.19.8"
"#;
        write(&path, cargo_toml)?;

        // Act
        let doc = read_toml_file_edit(&path)?;
        let result = doc.to_string();

        // Assert
        assert_eq!(cargo_toml, result);
        Ok(())
    }

    #[test]
    fn test_read_toml_file_edit_invalid_fails() -> Result<()> {
        // Arrange
        let temp_dir = TempDir::new("swiss-army-knife-test")?;
        let path = temp_dir.path().join("file.toml");
        write(&path, "xxx{\"message\": \"hello-world\"}")?;

        // Act
        let e = match read_toml_file_edit(&path) {
            Ok(_) => panic!("read_toml_file_edit must fail"),
            Err(e) => e,
        };

        // Assert
        assert_eq!(TomlErrorKind::Syntax, e.kind());
        assert!(e.is_syntax());
        assert!(!e.is_other());
        let message = format!("{}", e);
        assert!(message.contains(path.to_str().expect("must be valid string")));
        Ok(())
    }
}
