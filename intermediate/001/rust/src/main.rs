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

use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};
use std::collections::HashMap;

struct EventCalendar(HashMap<NaiveDateTime, Vec<(NaiveDateTime, String)>>);

impl EventCalendar {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn add_event(&mut self, start: NaiveDateTime, event: String) {
        let at_hour: NaiveDateTime =
            NaiveDate::from_ymd_opt(start.year(), start.month(), start.day())
                .unwrap()
                .and_hms_opt(start.hour(), 0, 0)
                .unwrap();
        self.0
            .entry(at_hour)
            .or_insert(Vec::new())
            .push((start, event));
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let calendar = EventCalendar::new();
        assert_eq!(0, calendar.0.len());
    }

    #[test]
    fn test_add_event() {
        let mut calendar = EventCalendar::new();
        let start: NaiveDateTime = NaiveDate::from_ymd_opt(2022, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let event = "Test Event".to_string();
        calendar.add_event(start, event.clone());
        assert_eq!(1, calendar.0.len());
        assert_eq!(1, calendar.0[&start].len());
        assert_eq!(start, calendar.0[&start][0].0);
        assert_eq!(event, calendar.0[&start][0].1);
    }
}
