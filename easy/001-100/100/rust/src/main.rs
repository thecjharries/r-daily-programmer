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

use chrono::{DateTime, Duration, TimeZone, Utc};

fn main() {
    println!("rad");
}

fn determine_sleep_start(sleep_end: DateTime<Utc>) -> Vec<DateTime<Utc>> {
    vec![
        sleep_end - Duration::minutes(6 * 90),
        sleep_end - Duration::minutes(5 * 90),
        sleep_end - Duration::minutes(4 * 90),
        sleep_end - Duration::minutes(3 * 90),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_sleep_start() {
        assert_eq!(
            determine_sleep_start(Utc.ymd(2020, 1, 1).and_hms(6, 15, 0)),
            vec![
                Utc.ymd(2019, 12, 31).and_hms(9, 15, 0),
                Utc.ymd(2019, 12, 31).and_hms(10, 45, 0),
                Utc.ymd(2020, 1, 1).and_hms(0, 15, 0),
                Utc.ymd(2020, 1, 1).and_hms(1, 45, 0)
            ]
        );
    }
}
