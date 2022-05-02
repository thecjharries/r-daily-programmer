// Copyright 2022 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use regex::Regex;

fn main() {
    println!("rad");
}

fn enforce_newlines(input: &str, is_windows: bool) -> String {
    let newline_pattern = Regex::new(r"\r?\n").unwrap();
    if is_windows {
        newline_pattern.replace_all(input, "\r\n").into_owned()
    } else {
        newline_pattern.replace_all(input, "\n").into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            enforce_newlines("test\r\none\ntwo", false),
            "test\none\ntwo"
        );
        assert_eq!(
            enforce_newlines("test\r\none\ntwo", true),
            "test\r\none\r\ntwo"
        );
    }
}
