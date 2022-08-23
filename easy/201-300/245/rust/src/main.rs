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
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MONTH_DAY_YEAR_PATTERN: Regex =
        Regex::new(r"^\s*(?P<month>\d{1,2})\D+(?P<day>\d{1,2})\D+(?P<year>\d{2}|\d{4})\s*$")
            .unwrap();
    static ref YEAR_MONTH_DAY_PATTERN: Regex =
        Regex::new(r"^\s*(?P<year>\d{4})\D+(?P<month>\d{1,2})\D+(?P<day>\d{1,2})\s*$").unwrap();
}

fn parse_date(input: &str) -> String {
    let captures = match MONTH_DAY_YEAR_PATTERN.is_match(input) {
        true => MONTH_DAY_YEAR_PATTERN.captures(input).unwrap(),
        false => YEAR_MONTH_DAY_PATTERN.captures(input).unwrap(),
    };
    let month: u32 = captures.name("month").unwrap().as_str().parse().unwrap();
    let day: u32 = captures.name("day").unwrap().as_str().parse().unwrap();
    let mut year: i32 = captures.name("year").unwrap().as_str().parse().unwrap();
    if 1900 > year {
        year += 2000;
    }
    let date = NaiveDate::from_ymd(year, month, day);
    date.format("%Y-%m-%d").to_string()
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date() {
        assert_eq!("2015-02-13".to_string(), parse_date("2/13/15"));
        assert_eq!("2010-01-31".to_string(), parse_date("1-31-10"));
        assert_eq!("2015-05-10".to_string(), parse_date("5 10 2015"));
        assert_eq!("2012-03-17".to_string(), parse_date("2012 3 17"));
        assert_eq!("2001-01-01".to_string(), parse_date("2001-01-01"));
        assert_eq!("2008-01-07".to_string(), parse_date("2008/01/07"));
    }
}
