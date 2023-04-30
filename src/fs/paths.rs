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
use std::path::{Component, Path};

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

#[cfg(test)]
mod tests {
    use super::{get_base_name, path_to_str};
    use rstest::rstest;
    use std::path::Path;

    // https://doc.rust-lang.org/stable/std/ffi/index.html#conversions
    #[cfg(not(target_os = "windows"))]
    fn make_path_containing_invalid_unicode() -> PathBuf {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;
        use std::path::PathBuf;
        const INVALID_UTF8: [u8; 1] = [192];
        PathBuf::from(OsString::from_vec(INVALID_UTF8.to_vec()))
    }

    #[rstest]
    #[case(Some("file"), Path::new("/path/to/file"))]
    #[case(None, Path::new(""))]
    fn base_name_basics(#[case] expected_base_name: Option<&str>, #[case] input: &Path) {
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
}
