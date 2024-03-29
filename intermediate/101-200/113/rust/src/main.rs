// Copyright 2023 CJ Harries
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

use lazy_static::lazy_static;
use regex::Regex;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn markup_bold(input: String) -> String {
    lazy_static! {
        static ref BOLD_REGEX: Regex = Regex::new(r"\*([^*]+)\*").unwrap();
    }
    BOLD_REGEX.replace_all(&input, "<b>$1</b>").to_string()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!("The string literal `<b>Test</b>` should print <b>Test</b> or <div style=\"font-weight:bold;\">Test</div>", markup_bold("The string literal `*Test*` should print <b>Test</b> or <div style=\"font-weight:bold;\">Test</div>".to_string()));
    }
}
