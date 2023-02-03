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

use chrono::Duration;
use chrono::NaiveDateTime;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn time_since(date: NaiveDateTime, unit: Duration) -> i64 {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_since() {
        let date =
            NaiveDateTime::parse_from_str("2000-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let unit = Duration::minutes(1);
        assert!(12088800 <= time_since(date, unit));
        let unit = Duration::hours(1);
        assert!(201480 <= time_since(date, unit));
        let unit = Duration::days(1);
        assert!(8395 <= time_since(date, unit));
        let unit = Duration::days(30);
        assert!(276 <= time_since(date, unit));
        let unit = Duration::days(365);
        assert!(23 <= time_since(date, unit));
    }
}
