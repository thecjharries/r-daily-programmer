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
fn determine_baum_term(number: u128) -> u8 {
    if 0 == number {
        return 1;
    }
    let number_binary = format!("{:b}", number);
    let mut current_zero_count = 0;
    for character in number_binary.chars() {
        if '0' == character {
            current_zero_count += 1;
        } else {
            if 1 == current_zero_count {
                return 0;
            }
            current_zero_count = 0;
        }
    }
    if 1 == current_zero_count {
        return 0;
    }
    1
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_baum_term() {
        assert_eq!(1, determine_baum_term(0));
        assert_eq!(1, determine_baum_term(4));
        assert_eq!(0, determine_baum_term(5));
        assert_eq!(0, determine_baum_term(19611206));
    }
}
