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

fn fibonacci_to_base10(input: &str) -> u32 {
    let mut base10 = 0;
    let mut current = 1;
    let mut previous = 1;
    if '1' == input.chars().last().unwrap() {
        base10 += 1;
    }
    if 1 < input.len() && '1' == input.chars().nth(input.len() - 2).unwrap() {
        base10 += 1;
    }
    for character in input.chars().rev().skip(2) {
        (previous, current) = (current, previous + current);
        if '1' == character {
            base10 += current;
        }
    }
    base10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_to_base10() {
        assert_eq!(1, fibonacci_to_base10("1"));
        assert_eq!(1, fibonacci_to_base10("10"));
        assert_eq!(20, fibonacci_to_base10("111111"));
        assert_eq!(8, fibonacci_to_base10("100000"));
        assert_eq!(2868, fibonacci_to_base10("10110110100111001"));
    }
}
