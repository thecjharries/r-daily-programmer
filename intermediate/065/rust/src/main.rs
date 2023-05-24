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

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Gender {
    Male,
    Female,
    Other,
}

impl FromStr for Gender {
    type Err = ();

    fn from_str(s: &str) -> Result<Gender, ()> {
        let character_string = s
            .chars()
            .filter(|character| character.is_alphabetic())
            .map(|character| character.to_lowercase().next().unwrap())
            .collect::<String>();
        if let Some(character) = character_string.chars().next() {
            match character {
                'm' => Ok(Gender::Male),
                'f' => Ok(Gender::Female),
                _ => Ok(Gender::Other),
            }
        } else {
            Err(())
        }
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
    fn test_gender_from_str() {
        assert_eq!(Gender::Male, "Male".parse::<Gender>().unwrap());
        assert_eq!(Gender::Female, Gender::from_str("(F)").unwrap());
        assert_eq!(Gender::Other, "qqq".parse::<Gender>().unwrap());
        assert!("!!!".parse::<Gender>().is_err());
    }
}
