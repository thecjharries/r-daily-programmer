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

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MONTH_DAY_YEAR_PATTERN: Regex =
        Regex::new(r"^\s*(?P<month>\d{1,2})\D+(?P<day>\d{1,2})\D+(?P<year>\d{2}|\d{4})\s*$")
            .unwrap();
    static ref YEAR_MONTH_DAY_PATTERN: Regex =
        Regex::new(r"^\s*(?P<year>\d{4})\D+(?P<month>\d{1,2})\D+(?P<day>\d{1,2})\s*$").unwrap();
}

fn convert_date(input: str) -> String {
    String::new()
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
