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

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
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

#[derive(Debug, PartialEq, Eq)]
struct Person {
    key: String,
    name: String,
    gender: Gender,
    parents: Vec<String>,
    children: Vec<String>,
}

impl FromStr for Person {
    type Err = ();

    fn from_str(s: &str) -> Result<Person, ()> {
        lazy_static! {
            static ref PERSON_PATTERN: Regex =
                Regex::new(r"(?P<key>[A-Z]{2})\s*=\s*(?P<name>.*)\s+?\((?P<gender>.)\)").unwrap();
        }
        let matches = PERSON_PATTERN.captures(s).unwrap();
        Ok(Person {
            key: matches.name("key").unwrap().as_str().to_string(),
            name: matches.name("name").unwrap().as_str().to_string(),
            gender: matches
                .name("gender")
                .unwrap()
                .as_str()
                .parse::<Gender>()
                .unwrap(),
            parents: Vec::new(),
            children: Vec::new(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct FamilyTree(HashMap<String, Person>);

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

    #[test]
    fn test_person_from_str() {
        // AA = Rickard Stark (M)
        let person = Person {
            key: "AA".to_string(),
            name: "Rickard Stark".to_string(),
            gender: Gender::Male,
            parents: Vec::new(),
            children: Vec::new(),
        };
        assert_eq!(person, "AA = Rickard Stark (M)".parse::<Person>().unwrap());
    }
}
