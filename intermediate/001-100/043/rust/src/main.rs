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
enum Day {
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

fn doomsday_day_of_week(year: u32, month: u32, day: u32) -> Day {
    let century = year / 100;
    let anchor = match century % 4 {
        0 => 2,
        1 => 0,
        2 => 5,
        _ => 3,
    };
    let year_digits = year % 100;
    let a = year_digits / 12;
    let b = year_digits % 12;
    let c = b / 4;
    let doomsday = (anchor + a + b + c) % 7;
    let is_leap_year = 0 == year % 4 && 0 != year % 100 || 0 == year % 400;
    let offset = if is_leap_year {
        match month {
            1 => 4,
            2 => 1,
            5 => 9,
            7 => 11,
            9 => 5,
            11 => 7,
            _ => 0,
        }
    } else {
        match month {
            1 => 3,
            5 => 9,
            7 => 11,
            9 => 5,
            11 => 7,
            _ => 0,
        }
    };
    let day_of_week = if month % 2 == 0 && month != 2 {
        (14 + doomsday + day - month) % 7
    } else {
        (14 + doomsday + day - offset) % 7
    };
    match day_of_week {
        0 => Day::Sunday,
        1 => Day::Monday,
        2 => Day::Tuesday,
        3 => Day::Wednesday,
        4 => Day::Thursday,
        5 => Day::Friday,
        _ => Day::Saturday,
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doomsday_day_of_week() {
        assert_eq!(Day::Tuesday, doomsday_day_of_week(2012, 4, 24));
        assert_eq!(Day::Saturday, doomsday_day_of_week(2012, 4, 21));
        assert_eq!(Day::Saturday, doomsday_day_of_week(2000, 4, 1));
        assert_eq!(Day::Sunday, doomsday_day_of_week(2001, 4, 1));
        assert_eq!(Day::Friday, doomsday_day_of_week(2023, 2, 24));
    }
}
