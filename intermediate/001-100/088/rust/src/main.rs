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

use chrono::{Datelike, NaiveDate};

const DIVIDER_LINE: &str = "+--------------------+";
const DAYS_LINE: &str = "|M |T |W |T |F |S |S |";

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn print_calendar(year: i32, month: u32) -> String {
    let date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let mut output = DIVIDER_LINE.to_string();
    output.push_str(&format!("\n|{: ^20}|", date.format("%B").to_string()));
    output.push_str(&format!("\n{}", DIVIDER_LINE));
    output.push_str(&format!("\n{}", DAYS_LINE));
    output.push_str(&format!("\n{}", DIVIDER_LINE));
    let mut current_date = date;
    let mut current_line = String::from("|");
    for _ in 0..current_date.weekday().num_days_from_monday() {
        current_line.push_str("  |");
    }
    while current_date.month() == month {
        current_line.push_str(&format!("{:>2}|", current_date.day()));
        if 6 == current_date.weekday().num_days_from_monday() {
            output.push_str(&format!("\n{}", current_line));
            current_line = String::from("|");
        }
        current_date = current_date.succ_opt().unwrap();
    }
    for _ in current_date.weekday().num_days_from_monday()..7 {
        current_line.push_str("  |");
    }
    output.push_str(&format!("\n{}", current_line));
    output.push_str(&format!("\n{}", DIVIDER_LINE));
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_calendar() {
        let output = "+--------------------+
|      January       |
+--------------------+
|M |T |W |T |F |S |S |
+--------------------+
|  |  |  |  |  |  | 1|
| 2| 3| 4| 5| 6| 7| 8|
| 9|10|11|12|13|14|15|
|16|17|18|19|20|21|22|
|23|24|25|26|27|28|29|
|30|31|  |  |  |  |  |
+--------------------+"
            .to_string();
        let result = print_calendar(2012, 1);
        let output_lines = output.lines().collect::<Vec<&str>>();
        let result_lines = result.lines().collect::<Vec<&str>>();
        for index in 0..output_lines.len() {
            assert_eq!(output_lines[index], result_lines[index]);
        }
        assert_eq!(output, print_calendar(2012, 1));
    }
}
