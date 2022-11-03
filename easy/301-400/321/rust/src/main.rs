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

static DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
static TEENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
static TENS: [&str; 5] = ["ten", "twenty", "thirty", "forty", "fifty"];

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn print_time(time: &str) -> String {
    let exploded_time: Vec<&str> = time.split(":").collect();
    let mut hour = exploded_time[0].parse::<usize>().unwrap();
    let mut ampm = "am";
    if 11 < hour {
        ampm = "pm";
        hour -= 12;
    }
    let mut hour_word = "twelve".to_string();
    if 0 < hour && 10 > hour {
        hour_word = DIGITS[hour - 1].to_string();
    } else if 10 < hour {
        hour_word = TENS[hour - 11].to_string();
    }
    let minute = exploded_time[1].parse::<usize>().unwrap();
    let mut minute_word;
    if 0 == minute {
        minute_word = "".to_string();
    } else if 10 > minute {
        minute_word = format!("oh {} ", DIGITS[minute - 1]);
    } else if 20 > minute {
        minute_word = format!("{} ", TEENS[minute - 10]);
    } else {
        minute_word = format!("{} ", TENS[minute / 10 - 1]);
        if 0 != minute % 10 {
            minute_word = format!("{}{} ", minute_word, DIGITS[minute % 10 - 1]);
        }
    }
    format!("It's {} {}{}", hour_word, minute_word, ampm)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!("It's twelve am".to_string(), print_time("00:00"));
        assert_eq!("It's one thirty am".to_string(), print_time("01:30"));
        assert_eq!("It's twelve oh five pm".to_string(), print_time("12:05"));
        assert_eq!("It's two oh one pm".to_string(), print_time("14:01"));
        assert_eq!("It's eight twenty nine pm".to_string(), print_time("20:29"));
        assert_eq!("It's nine pm".to_string(), print_time("21:00"));
    }
}
