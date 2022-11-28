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

const BASE_62_ALPHABET: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn convert_to_base_62(number: u128) -> String {
    let mut result = String::new();
    let mut remainder = number;
    while remainder > 0 {
        result.push(
            BASE_62_ALPHABET
                .chars()
                .nth((remainder % 62) as usize)
                .unwrap(),
        );
        remainder /= 62;
    }
    result.chars().collect()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_base_62() {
        assert_eq!("O44", convert_to_base_62(15674));
        assert_eq!("bDcRfbr63n8", convert_to_base_62(7026425611433322325));
        assert_eq!("9OM", convert_to_base_62(187621));
        assert_eq!("3n26g", convert_to_base_62(237860461));
        assert_eq!("B4b9", convert_to_base_62(2187521));
        assert_eq!("sS4", convert_to_base_62(18752));
    }
}
