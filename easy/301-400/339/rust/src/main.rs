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

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INFO_PATTERN: Regex =
        Regex::new(r"^(?P<name>[^:].{19})(?P<age>\d{2})(?P<birthday>\d{6})$").unwrap();
    static ref EXTENSION_PATTERN: Regex =
        Regex::new(r"::EXT::(?P<name>.{4})(?P<value>.{17})").unwrap();
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_highest_salary(input: &str) -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_highest_salary() {
        assert_eq!(
            "Marcelo Candela, $47706".to_string(),
            find_highest_salary(
                "Boyce Calles        83460319
::EXT::SAL 00000000000044722
Marcelo Candela     29040821
::EXT::JOB loser
::EXT::SAL 00000000000047706
Milton Camper       32541106"
            )
        );
    }
}
