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

fn get_st_pats_day_of_week_from_year(year: i32) -> String {
    let date = NaiveDate::from_ymd_opt(year, 3, 17).unwrap();
    date.format("%A").to_string()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            "Wednesday".to_string(),
            get_st_pats_day_of_week_from_year(2021)
        );
        assert_eq!(
            "Tuesday".to_string(),
            get_st_pats_day_of_week_from_year(2020)
        );
        assert_eq!(
            "Friday".to_string(),
            get_st_pats_day_of_week_from_year(2000)
        );
        assert_eq!(
            "Saturday".to_string(),
            get_st_pats_day_of_week_from_year(1900)
        );
    }
}
