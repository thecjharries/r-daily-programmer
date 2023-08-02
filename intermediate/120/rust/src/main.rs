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

const DIGITS: [char; 64] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z', '+', '/',
];

#[derive(Debug)]
struct Base {
    digits: Vec<char>,
}

impl Base {
    fn new(size: u8) -> Self {
        Self {
            digits: DIGITS[..size as usize].to_vec(),
        }
    }

    fn to_base_ten(&self, number: &str) -> String {
        let mut result = 0;
        for (index, character) in number.chars().rev().enumerate() {
            result += self.digits.iter().position(|&x| x == character).unwrap()
                * self.digits.len().pow(index as u32);
        }
        result.to_string()
    }

    fn from_base_ten(&self, number: &str) -> String {
        let mut result = String::new();
        let mut number = number.parse::<u32>().unwrap();
        while number > 0 {
            result.push(self.digits[(number % self.digits.len() as u32) as usize]);
            number /= self.digits.len() as u32;
        }
        result.chars().rev().collect()
    }

    fn from_base(&self, number: &str, base: &Base) -> String {
        self.from_base_ten(&base.to_base_ten(number))
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
    fn base_new_generates_proper_base() {
        assert_eq!(Base::new(2).digits, vec!['0', '1'],);
    }

    #[test]
    fn each_base_can_convert_to_base_ten() {
        assert_eq!(Base::new(2).to_base_ten("10"), "2");
        assert_eq!(Base::new(2).to_base_ten("11"), "3");
        assert_eq!(Base::new(2).to_base_ten("100"), "4");
        assert_eq!(Base::new(64).to_base_ten("10"), "64");
    }

    #[test]
    fn each_base_can_convert_from_base_ten() {
        assert_eq!(Base::new(2).from_base_ten("2"), "10");
        assert_eq!(Base::new(2).from_base_ten("3"), "11");
        assert_eq!(Base::new(2).from_base_ten("4"), "100");
        assert_eq!(Base::new(64).from_base_ten("64"), "10");
    }

    #[test]
    fn each_base_can_convert_from_other_bases() {
        assert_eq!(Base::new(2).from_base("10", &Base::new(2)), "10");
        assert_eq!(Base::new(2).from_base("11", &Base::new(2)), "11");
        assert_eq!(Base::new(2).from_base("100", &Base::new(2)), "100");
        assert_eq!(Base::new(64).from_base("10", &Base::new(64)), "10");
        assert_eq!(Base::new(22).from_base("e1f1", &Base::new(25)), "ke0f");
    }
}
