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
use colored::Colorize;
use serde_json::Value;
use std::process::exit;
use swiss_army_knife::{read_json_file, safe_write_file, FileError, JsonError};
use tempdir::TempDir;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn run() -> Result<()> {
    let temp_dir = TempDir::new("swiss-army-knife")?;
    let temp_file_path = temp_dir.path().join("temp.json");
    safe_write_file(&temp_file_path, "\"xyz", false)?;
    let value = read_json_file::<Value, _>(&temp_file_path)?;
    println!("value={:#?}", value);
    Ok(())
}

fn main() {
    exit(match run() {
        Ok(()) => 0,
        Err(e) => {
            if let Some(file_error) = e.downcast_ref::<FileError>() {
                println!("{}", format!("File error: {}", file_error).bright_red());
            } else if let Some(json_error) = e.downcast_ref::<JsonError>() {
                println!("{}", format!("JSON error: {}", json_error).bright_red());
            } else {
                println!("{}", format!("Unhandled error: {:#?}", e).red());
            }
            1
        }
    })
}
