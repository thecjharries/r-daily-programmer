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

use std::collections::HashMap;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_repeated_numbers(input: &str) -> HashMap<String, u32> {
    let mut repeated: HashMap<String, u32> = HashMap::new();
    for length in 2..input.len() {
        for index in 0..input.len() - length {
            let number = input[index..index + length].to_string();
            repeated
                .entry(number)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
    let mut output: HashMap<String, u32> = HashMap::new();
    for (number, count) in repeated.iter_mut() {
        if 1 < *count {
            output.insert(number.to_string(), *count);
        }
    }
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            HashMap::from([
                ("21".to_string(), 2),
                ("23".to_string(), 2),
                ("232".to_string(), 2),
                ("25".to_string(), 2),
                ("32".to_string(), 4),
                ("321".to_string(), 2),
                ("325".to_string(), 2)
            ]),
            find_repeated_numbers("11325992321982432123259")
        );
    }
}
