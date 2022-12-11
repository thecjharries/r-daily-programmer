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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(not(tarpaulin_include))]
fn find_upc_check_digit(upc: &str) -> u8 {
    if 11 != upc.len() {
        return 0;
    }
    let mut even_sum = 0;
    let mut odd_sum = 0;
    for (index, character) in upc.chars().enumerate() {
        if 0 == index % 2 {
            even_sum += u32::from(character.to_digit(10).unwrap());
        } else {
            odd_sum += u32::from(character.to_digit(10).unwrap());
        }
    }
    let digit = ((even_sum * 3) + odd_sum) % 10;
    if 0 == digit {
        0
    } else {
        (10 - digit) as u8
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(4, find_upc_check_digit("04210000526"));
        assert_eq!(2, find_upc_check_digit("03600029145"));
        assert_eq!(4, find_upc_check_digit("12345678910"));
        assert_eq!(0, find_upc_check_digit("1234567"));
    }
}
