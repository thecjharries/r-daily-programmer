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

use std::collection::HashMap;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn compare_prices(
    first: HashMap<String, i32>,
    second: HashMap<String, i32>,
) -> HashMap<String, String> {
    HashMap::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_prices() {
        assert_eq!(
            compare_prices(
                HashMap::from_iter([
                    ("CarriageBolt".to_string(), 45),
                    ("Eyebolt".to_string(), 50),
                    ("Washer".to_string(), 120),
                    ("Rivet".to_string(), 10),
                ]),
                HashMap::from_iter([
                    ("CarriageBolt".to_string(), 45),
                    ("Eyebolt".to_string(), 45),
                    ("Washer".to_string(), 140),
                    ("Rivet".to_string(), 10),
                ])
            ),
            HashMap::from_iter([
                ("Eyebolt".to_string(), "-5".to_string()),
                ("Washer".to_string(), "+20".to_string()),
            ])
        );
        assert_eq!(
            compare_prices(
                HashMap::from_iter([
                    ("2DNail".to_string(), 3),
                    ("4DNail".to_string(), 5),
                    ("8DNail".to_string(), 10),
                ]),
                HashMap::from_iter([
                    ("8DNail".to_string(), 11),
                    ("4DNail".to_string(), 5),
                    ("2DNail".to_string(), 2),
                ])
            ),
            HashMap::from_iter([
                ("2DNail".to_string(), "-1".to_string()),
                ("8DNail".to_string(), "+1".to_string()),
            ])
        );
    }
}
