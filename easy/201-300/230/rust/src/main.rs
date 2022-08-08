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

use serde_json::Value;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(not(tarpaulin_include))]
fn find_string(haystack: &str, needle: &str) -> String {
    let json: Value = serde_json::from_str(haystack).unwrap();
    for value in json.as_object().unwrap() {
        let (key, val) = value;
        match val.as_array() {
            Some(vec) => {
                for (k, v) in vec.iter().enumerate() {
                    if v.as_str().unwrap() == needle {
                        return format!("{} -> {}", key.to_string(), k);
                    }
                }
            }
            None => match val.as_str() {
                Some(val) => {
                    if val == needle {
                        return key.to_string();
                    }
                }
                None => {
                    continue;
                }
            },
        }
    }
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            "favoriteWebsites -> 1".to_string(),
            find_string(
                r#"{"name": "William Shakespeare", "wife": {"birthYear": 1555, "deathYear":
"Fun fact, she's a vampire", "name": "Anne Hathaway", "dead": false},
"favoriteWebsites": ["dailysonneter", "dailyprogrammer",
"vine (he's way into 6-second cat videos)"], "dead": true, "birthYear": 1564,
"facebookProfile": null, "selectedWorks": [{"written": 1606, "name":
"The Tragedy of Macbeth", "isItAwesome": true}, {"written": 1608, "name":
"Coriolanus", "isItAwesome": "It's alright, but kinda fascist-y"}], "deathYear":
 1616}"#,
                "dailyprogrammer"
            )
        );
    }
}
