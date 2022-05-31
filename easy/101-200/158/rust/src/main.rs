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

fn is_torn_number(input: u32) -> bool {
    if input > 999 && input < 10000 {
        let upper = input / 100;
        let lower = input - upper * 100;
        return input == (upper + lower) * (upper + lower);
    }
    false
}

fn find_torn_numbers() -> Vec<u32> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_torn_number() {
        assert_eq!(false, is_torn_number(1111));
        assert_eq!(false, is_torn_number(111));
        assert_eq!(false, is_torn_number(11111));
        assert_eq!(true, is_torn_number(3025));
        assert_eq!(true, is_torn_number(9801));
    }

    #[test]
    fn test_find_torn_numbers() {
        assert_eq!(vec![3025, 9801], find_torn_numbers());
    }
}
