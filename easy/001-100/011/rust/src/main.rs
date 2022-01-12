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

use chrono::{NaiveDate, Weekday};

fn main() {
    println!("rad");
}

fn get_day_of_week(year: i32, month: i32, day: i32) -> Weekday {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_day_of_week() {
        assert_eq!(get_day_of_week(2020, 1, 1), Weekday::Mon);
        assert_eq!(get_day_of_week(2020, 1, 2), Weekday::Tue);
        assert_eq!(get_day_of_week(2020, 1, 3), Weekday::Wed);
    }
}
