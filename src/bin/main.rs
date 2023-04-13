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
use swiss_army_knife::{
    read_json_file, read_toml_file, read_yaml_file, safe_write_file, FileError, JsonError,
    TomlError, YamlError,
};
use tempdir::TempDir;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn run() -> Result<()> {
    let temp_dir = TempDir::new("swiss-army-knife")?;

    let json_file_path = temp_dir.path().join("temp.json");
    safe_write_file(&json_file_path, "{\"name\": 123}", false)?;
    let value = read_json_file::<Value, _>(&json_file_path)?;
    println!("value={:#?}", value);

    let yaml_file_path = temp_dir.path().join("temp.yaml");
    safe_write_file(&yaml_file_path, "name: 123", false)?;
    let value = read_yaml_file::<Value, _>(&yaml_file_path)?;
    println!("value={:#?}", value);

    let toml_file_path = temp_dir.path().join("temp.toml");
    safe_write_file(&toml_file_path, "name = 123", false)?;
    let value = read_toml_file::<Value, _>(&toml_file_path)?;
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
            } else if let Some(yaml_error) = e.downcast_ref::<YamlError>() {
                println!("{}", format!("YAML error: {}", yaml_error).bright_red());
            } else if let Some(toml_error) = e.downcast_ref::<TomlError>() {
                println!("{}", format!("TOML error: {}", toml_error).bright_red());
            } else {
                println!("{}", format!("Unhandled error: {:#?}", e).red());
            }
            1
        }
    })
}
