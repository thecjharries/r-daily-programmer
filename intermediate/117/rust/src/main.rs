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

use chrono::NaiveDate;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn get_mayan_long_count(year: i32, month: u32, day: u32) -> String {
    let date = NaiveDate::from_ymd(year, month, day);
    let days_since_epoch = date
        .signed_duration_since(NaiveDate::from_ymd(1970, 1, 1))
        .num_days()
        + 12 * 144000
        + 17 * 7200
        + 16 * 360
        + 7 * 20
        + 5;
    let baktun = days_since_epoch / 144000;
    let katun = (days_since_epoch % 144000) / 7200;
    let tun = ((days_since_epoch % 144000) % 7200) / 360;
    let uinal = (((days_since_epoch % 144000) % 7200) % 360) / 20;
    let kin = (((days_since_epoch % 144000) % 7200) % 360) % 20;
    format!("{}.{}.{}.{}.{}", baktun, katun, tun, uinal, kin)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mayan_long_count() {
        assert_eq!("12.17.16.7.5", get_mayan_long_count(1970, 1, 1));
        assert_eq!("12.18.15.4.0", get_mayan_long_count(1988, 7, 20));
        assert_eq!("12.19.19.17.11", get_mayan_long_count(2012, 12, 12));
    }
}
