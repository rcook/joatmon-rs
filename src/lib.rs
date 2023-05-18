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
mod error;
mod formats;
mod fs;

pub use self::error::HasOtherError;
pub use self::formats::{
    read_json_file, read_toml_file, read_toml_file_edit, read_yaml_file, JsonError, JsonErrorKind,
    TomlError, TomlErrorKind, YamlError, YamlErrorKind,
};
pub use self::fs::{
    file_name_safe_timestamp, find_sentinel_dir, find_sentinel_file, label_file_name, open_file,
    read_bytes, read_text_file, safe_back_up, safe_create_file, safe_write_file, FileReadError,
    FileReadErrorKind, FileWriteError, FileWriteErrorKind, WorkingDirectory,
};
