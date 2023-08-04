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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn do_julian_years_and_lunar_months_give_same_days(
    julian_years: u32,
    lunar_months: u32,
) -> Option<u64> {
    let lunar_days = (29.53059 * lunar_months as f32) as u64;
    let julian_days = 365 * julian_years as u64 + julian_years as u64 / 4;
    if lunar_days == julian_days {
        Some(lunar_days)
    } else {
        None
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_julian_years_and_lunar_months_give_same_days() {
        assert_eq!(
            Some(13879),
            do_julian_years_and_lunar_months_give_same_days(38, 470)
        );
        assert_eq!(
            Some(41638),
            do_julian_years_and_lunar_months_give_same_days(114, 2664)
        );
        assert_eq!(
            Some(0),
            do_julian_years_and_lunar_months_give_same_days(30, 82)
        );
    }
}
