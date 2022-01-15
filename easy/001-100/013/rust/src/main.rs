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

use chrono::{NaiveDate, Datelike};

fn main() {
    println!("rad");
}

fn calculate_day_of_year(year: i32, month: u32, day: u32) -> u32 {
    let date = NaiveDate::from_ymd(year, month, day);
    date.ordinal()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_day_of_year() {
        assert_eq!(calculate_day_of_year(2019, 1, 1), 1);
        assert_eq!(calculate_day_of_year(2019, 3, 1), 60);
        assert_eq!(calculate_day_of_year(2020, 1, 1), 1);
        assert_eq!(calculate_day_of_year(2020, 3, 1), 61);
    }
}
