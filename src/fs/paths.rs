// The MIT License (MIT)
//
// Copyright (c) 2020-3 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
use chrono::{DateTime, SecondsFormat, Utc};
use std::ffi::OsString;
use std::path::{Component, Path, PathBuf};

pub fn path_to_str(path: &Path) -> &str {
    path.to_str()
        .expect("Path contains invalid Unicode characters")
}

pub fn get_base_name(path: &Path) -> Option<&str> {
    path.components().last().and_then(|x| {
        if let Component::Normal(s) = x {
            s.to_str()
        } else {
            None
        }
    })
}

pub fn label_file_name(path: &Path, label: &str) -> Option<PathBuf> {
    let mut file_name = OsString::new();

    if let Some(s) = path.file_stem() {
        file_name.push(s);
    } else {
        return None;
    }

    file_name.push("-");
    file_name.push(label);

    if let Some(s) = path.extension() {
        file_name.push(".");
        file_name.push(s)
    }

    Some(path.with_file_name(file_name))
}

pub fn file_name_safe_timestamp(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339_opts(SecondsFormat::Millis, true)
        .replace(['-', ':', '.'], "")
}

#[cfg(test)]
mod tests {
    use super::{file_name_safe_timestamp, get_base_name, label_file_name, path_to_str};
    use chrono::{TimeZone, Utc};
    use rstest::rstest;
    use std::path::{Path, PathBuf};

    // https://doc.rust-lang.org/stable/std/ffi/index.html#conversions
    #[cfg(not(target_os = "windows"))]
    fn make_path_containing_invalid_unicode() -> std::path::PathBuf {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;
        const INVALID_UTF8: [u8; 1] = [192];
        PathBuf::from(OsString::from_vec(INVALID_UTF8.to_vec()))
    }

    #[rstest]
    #[case(Some("file"), Path::new("/path/to/file"))]
    #[case(None, Path::new(""))]
    #[case(None, Path::new("/"))]
    fn get_base_name_basics(#[case] expected_base_name: Option<&str>, #[case] input: &Path) {
        assert_eq!(expected_base_name, get_base_name(input));
    }

    #[rstest]
    #[case("/path/to/file", Path::new("/path/to/file"))]
    fn path_to_str_basics(#[case] expected_str: &str, #[case] input: &Path) {
        assert_eq!(expected_str, path_to_str(input));
    }

    #[test]
    #[should_panic]
    #[cfg(not(target_os = "windows"))]
    fn path_to_str_invalid_unicode_panics() {
        let path = make_path_containing_invalid_unicode();
        _ = path_to_str(&path)
    }

    #[rstest]
    #[case(Some(PathBuf::from("/aaa/bbb/ccc-ddd.txt")), "/aaa/bbb/ccc.txt", "ddd")]
    #[case(Some(PathBuf::from("/aaa/bbb/ccc-ddd")), "/aaa/bbb/ccc", "ddd")]
    #[case(Some(PathBuf::from("ccc-ddd.txt")), "ccc.txt", "ddd")]
    #[case(Some(PathBuf::from("ccc-ddd")), "ccc", "ddd")]
    fn label_file_name_basics(
        #[case] expected_path: Option<PathBuf>,
        #[case] path: PathBuf,
        #[case] label: &str,
    ) {
        assert_eq!(expected_path, label_file_name(&path, label))
    }

    #[test]
    fn file_name_safe_timestamp_basics() {
        let dt = Utc
            .with_ymd_and_hms(2019, 3, 17, 16, 43, 0)
            .single()
            .expect("must be valid");
        assert_eq!("20190317T164300000Z", file_name_safe_timestamp(&dt));
    }
}
