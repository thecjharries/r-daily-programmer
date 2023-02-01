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

lazy_static! {
    static ref LINE_PATTERN: Regex =
        Regex::new(r"(?P<name>\w+)(?:\s+\((?P<collection>[^)]+)\))?.*").unwrap();
    static ref VALUE_PATTERN: Regex = Regex::new(r"\[(?P<key>\w)\](?P<value>.*)").unwrap();
}

#[derive(Debug, PartialEq)]
struct PromptCollection {
    name: String,
    collection: HashMap<char, String>,
}

impl FromStr for PromptCollection {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut name = String::new();
        let mut collection = HashMap::new();
        if let Some(captures) = LINE_PATTERN.captures(input) {
            name = captures.name("name").unwrap().as_str().to_string();
            if let Some(collection_string) = captures.name("collection") {
                for value in collection_string.as_str().split(",") {
                    if let Some(captures) = VALUE_PATTERN.captures(value) {
                        let key = captures
                            .name("key")
                            .unwrap()
                            .as_str()
                            .chars()
                            .next()
                            .unwrap();
                        let mut value = String::new();
                        value.push(key);
                        value.push_str(captures.name("value").unwrap().as_str().trim());
                        collection.insert(key.to_ascii_lowercase(), value);
                    }
                }
            }
        }
        Ok(PromptCollection { name, collection })
    }
}

impl PromptCollection {
    fn to_html(&mut self) -> String {
        todo!()
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
    fn test_prompt_collection_from_str() {
        let expected = PromptCollection {
            name: "Gender".to_string(),
            collection: vec![('m', "Male".to_string()), ('f', "Female".to_string())]
                .into_iter()
                .collect(),
        };
        assert_eq!(
            expected,
            PromptCollection::from_str("Gender ([M]ale, [F]emale):").unwrap()
        );
        let expected = PromptCollection {
            name: "Name".to_string(),
            collection: HashMap::new(),
        };
        assert_eq!(expected, PromptCollection::from_str("Name:").unwrap());
    }

    #[test]
    fn test_prompt_collection_to_html_one() {
        let collection = PromptCollection {
            name: "Name".to_string(),
            collection: HashMap::new(),
        };
        let result = r#"Name:
<input type="text" name="name"/>"#;
        assert_eq!(result, collection.to_html());
    }

    #[test]
    fn_test_prompt_collection_to_html_less_than_five() {
        let collection = PromptCollection {
            name: "Gender".to_string(),
            collection: vec![('m', "Male".to_string()), ('f', "Female".to_string())]
                .into_iter()
                .collect(),
        };
        let result = r#"Gender:
<input type="radio" name="gender" value="m"/> Male
<input type="radio" name="gender" value="f"/> Female"#;
        assert_eq!(result, collection.to_html());
    }
}
