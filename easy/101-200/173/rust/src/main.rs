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
use std::collections::HashMap;

lazy_static! {
    static ref CONVERSION_PATTERN: Regex =
        Regex::new(r"\s*(?P<amount>[^ ]*) (?P<from>.*) to (?P<to>.*)\s*").unwrap();
    static ref CONVERSION_MAP: HashMap<String, HashMap<String, f64>> = {
        let mut map = HashMap::new();
        map.insert(
            "inches".to_string(),
            HashMap::from_iter([
                ("metres".to_string(), 1.0 / 39.37),
                ("miles".to_string(), 1.0 / 63360.0),
                ("attoparsecs".to_string(), 1.0 / 1.215),
            ]),
        );
        map.insert(
            "metres".to_string(),
            HashMap::from_iter([
                ("inches".to_string(), 39.37),
                ("miles".to_string(), 1.0 / 1609.0),
                ("attoparsecs".to_string(), 32.408),
            ]),
        );
        map.insert(
            "miles".to_string(),
            HashMap::from_iter([
                ("inches".to_string(), 63360.0),
                ("metres".to_string(), 1609.0),
                ("attoparsecs".to_string(), 52155.0),
            ]),
        );
        map.insert(
            "attoparsecs".to_string(),
            HashMap::from_iter([
                ("inches".to_string(), 1.0 / 1.215),
                ("metres".to_string(), 1.0 / 32.408),
                ("miles".to_string(), 1.0 / 52155.0),
            ]),
        );
        map.insert(
            "kilograms".to_string(),
            HashMap::from_iter([
                ("pounds".to_string(), 0.45359237),
                ("ounces".to_string(), 28.349523125),
            ]),
        );
        map
    };
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn convert_units(input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
