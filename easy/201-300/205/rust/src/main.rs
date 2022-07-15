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

use chrono::{Datelike, NaiveDate, Utc};

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn build_pretty_date_diff(first: &str, second: &str) -> String {
    let first_date = NaiveDate::parse_from_str(first, "%Y-%m-%d").unwrap();
    let second_date = NaiveDate::parse_from_str(second, "%Y-%m-%d").unwrap();
    let diff = second_date.signed_duration_since(first_date);
    if 0 == diff.num_days() {
        return first_date
            .format("%B %e, %Y")
            .to_string()
            .replace("  ", " ");
    }
    format!(
        "{} - {}",
        first_date.format("%B %e"),
        second_date.format("%e")
    )
    .replace("  ", " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_pretty_date_diff() {
        let current_year = Utc::now().year();
        let mut first = format!("{}-07-01", current_year);
        let mut second = format!("{}-07-04", current_year);
        let mut output = "July 1 - 4".to_string();
        assert_eq!(output, build_pretty_date_diff(&first, &second));
        first = format!("{}-07-01", current_year);
        second = format!("{}-07-01", current_year);
        output = format!("July 1, {}", current_year);
        assert_eq!(output, build_pretty_date_diff(&first, &second));
    }
}
