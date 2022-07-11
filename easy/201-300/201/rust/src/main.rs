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

use chrono::{Duration, NaiveDate};

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn days_until(year: i32, month: i32, day: i32) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_days_until() {
        let now = NaiveDate::now();
        assert_eq!(days_until(now.year(), now.month(), now.day()), 0);
        let future = NaiveDate::from_ymd(now.year(), now.month(), now.days()) + Duration::days(60);
        assert_eq!(days_until(future.year(), future.month(), future.day()), 60);
        let past = NaiveDate::from_ymd(now.year(), now.month(), now.days()) - Duration::days(15);
        assert_eq!(days_until(past.year(), past.month(), past.day()), -15);
    }
}
