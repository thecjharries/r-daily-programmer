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

fn xor_multiplication(first: u32, second: u32) -> u32 {
    let mut result = 0;
    let mut first_binary = format!("{:b}", first);
    let second_binary = format!("{:b}", second).chars().rev().collect::<Vec<char>>();
    for character in second_binary {
        if '1' == character {
            result ^= u32::from_str_radix(&first_binary, 2).unwrap();
        }
        first_binary.push('0');
    }
    result
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2, xor_multiplication(1, 2));
        assert_eq!(0, xor_multiplication(9, 0));
        assert_eq!(127, xor_multiplication(13, 11));
    }
}
