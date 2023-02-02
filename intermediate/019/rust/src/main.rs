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

lazy_static! {
    static ref TITLE_LINE_PATTERN: Regex =
        Regex::new(r"(?:ADVENTURE\s+)?[XVI]+.\s+(?P<title>.*)").unwrap();
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn is_title_line(input: &str) -> bool {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_title_line() {
        assert_eq!(
            false,
            is_title_line("On glancing over my notes of the seventy odd cases in which I")
        );
        assert_eq!(true, is_title_line("ADVENTURE I. A SCANDAL IN BOHEMIA"));
        assert_eq!(true, is_title_line("X. THE NORWOOD BUILDER"));
    }
}
