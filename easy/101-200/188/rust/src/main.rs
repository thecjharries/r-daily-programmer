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

use chrono::NaiveDate;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn correct_date_to_iso8601(input: &str) -> String {
    let possible_formats = vec![
        "%Y-%m-%d",
        "%m/%d/%y",
        "%m#%y#%d",
        "%d\\*%m\\*%Y",
        "%b %d, %y",
        "%b %d, %Y",
    ];
    for format in possible_formats {
        let parsed = NaiveDate::parse_from_str(input, format);
        if let Ok(date) = parsed {
            return date.format("%Y-%m-%d").to_string();
        }
    }
    panic!("Could not parse date: {}", input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_date_to_iso8601() {
        assert_eq!("2022-06-01", correct_date_to_iso8601("Jun 1, 22"));
    }

    #[test]
    #[should_panic]
    fn test_correct_date_to_iso8601_panic() {
        correct_date_to_iso8601("June 1, 2020");
    }
}
