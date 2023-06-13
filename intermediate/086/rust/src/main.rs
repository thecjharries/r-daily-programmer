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

#[derive(Debug, PartialEq)]
#[repr(u8)]
enum DayOfWeek {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn calculate_day_of_week(year: u16, month: u16, day: u16) -> DayOfWeek {
    let century = year / 100;
    let year_of_century = year % 100;
    let anchor = (5 * (century % 4) + 2) % 7;
    let a = year_of_century / 12;
    let b = year_of_century % 12;
    let c = b / 4;
    let doomsday = (a + b + c + anchor) % 7;
    let is_leap_year = 0 == year % 4 && (0 != year % 100 || 0 == year % 400);
    let anchor_day = if is_leap_year {
        match month {
            1 => 4,
            2 => 1,
            3 => 7,
            4 => 4,
            5 => 2,
            6 => 6,
            7 => 4,
            8 => 1,
            9 => 5,
            10 => 3,
            11 => 7,
            12 => 5,
            _ => 0,
        }
    } else {
        match month {
            1 => 3,
            2 => 7,
            3 => 7,
            4 => 4,
            5 => 2,
            6 => 6,
            7 => 4,
            8 => 1,
            9 => 5,
            10 => 3,
            11 => 7,
            12 => 5,
            _ => 0,
        }
    };
    let day_of_week = (7 + doomsday + day - anchor_day) % 7;
    match day_of_week {
        0 => DayOfWeek::Sunday,
        1 => DayOfWeek::Monday,
        2 => DayOfWeek::Tuesday,
        3 => DayOfWeek::Wednesday,
        4 => DayOfWeek::Thursday,
        5 => DayOfWeek::Friday,
        6 => DayOfWeek::Saturday,
        _ => DayOfWeek::Sunday,
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_day_of_week() {
        assert_eq!(DayOfWeek::Tuesday, calculate_day_of_week(2023, 6, 13));
        assert_eq!(DayOfWeek::Wednesday, calculate_day_of_week(2020, 1, 1));
    }
}
