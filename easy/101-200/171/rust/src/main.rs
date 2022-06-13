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

use hex;
use std::iter::FromIterator;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn convert_to_hex_picture(input: &str) -> String {
    let mut result = String::new();
    let cleaned_hex = input.replace(" ", "").chars().collect::<Vec<char>>();
    for index in (0..cleaned_hex.len()).step_by(2) {
        result.push_str(
            &format!(
                "{:#010b}\n",
                hex::decode(String::from_iter(&cleaned_hex[index..index + 2])).unwrap()[0]
            )
            .replace("0b", "")
            .replace("0", " ")
            .replace("1", "x"),
        );
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_hex_picture() {
        assert_eq!(
            "   xx   \n  xxxx  \n xxxxxx \n xxxxxx \n   xx   \n   xx   \n   xx   \n   xx    \n",
            convert_to_hex_picture("18 3C 7E 7E 18 18 18 18")
        );
    }
}
