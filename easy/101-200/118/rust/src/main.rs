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

use chrono::{DateTime, TimeZone};
use std::fmt::Display;

fn main() {
    println!("rad");
}

fn format_time<Tz: TimeZone>(format: &str, time: DateTime<Tz>) -> String
where
    <Tz as TimeZone>::Offset: Display,
{
    let mut result = String::from(format);
    result = result.replace(
        "%l",
        (&time.format("%f").to_string().parse::<i32>().unwrap() / 1000000)
            .to_string()
            .as_str(),
    );
    result = result.replace("%s", &time.format("%S").to_string());
    result = result.replace("%m", &time.format("%M").to_string());
    result = result.replace("%h", &time.format("%l").to_string());
    result = result.replace("%H", &time.format("%H").to_string());
    result = result.replace("%c", &time.format("%p").to_string());
    result = result.replace("%d", &time.format("%e").to_string());
    result = result.replace("%M", &time.format("%m").to_string());
    result = result.replace("%y", &time.format("%Y").to_string());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time() {
        let datetime = DateTime::parse_from_rfc3339("2021-04-28T17:41:44.68072657-05:00").unwrap();
        assert_eq!(
            format_time("%Y-%m-%d %H:%M:%S", datetime),
            "2021-04-28 17:41:44"
        );
    }
}
