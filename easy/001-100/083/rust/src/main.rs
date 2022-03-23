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

lazy_static! {
    static ref SCALE: Vec<(u128, String, String)> = vec![
        (
            1000000000000000000,
            "quintillion".to_string(),
            "trillion".to_string()
        ),
        (
            1000000000000000,
            "quadrillion".to_string(),
            "billiard".to_string()
        ),
        (1000000000000, "trillion".to_string(), "billion".to_string()),
        (1000000000, "billion".to_string(), "milliard".to_string()),
        (1000000, "million".to_string(), "million".to_string()),
        (1000, "thousand".to_string(), "thousand".to_string()),
        (1, "".to_string(), "".to_string()),
    ];
}

fn main() {
    println!("rad");
}

fn build_representation(number: u128) -> (String, String) {
    (format!("{}", number), format!("{}", number))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_representation() {
        assert_eq!(
            build_representation(1234567891111),
            (
                "1 trillion, 234 billion, 567 million, 891 thousand, and 111".to_string(),
                "1 billion, 234 milliard, 567 million, 891 thousand, and 111".to_string()
            )
        )
    }
}
