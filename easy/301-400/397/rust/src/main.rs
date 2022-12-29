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

static NUMERAL_ORDER: &str = "MDCLXVI";

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn numcompare(first: &str, second: &str) -> bool {
    let max_length = first.len().max(second.len());
    for index in 0..max_length {
        if first.len() <= index {
            return true;
        }
        if second.len() <= index {
            return false;
        }
        let first_value = NUMERAL_ORDER
            .find(first.chars().nth(index).unwrap())
            .unwrap();
        let second_value = NUMERAL_ORDER
            .find(second.chars().nth(index).unwrap())
            .unwrap();
        if first_value < second_value {
            return false;
        }
    }
    false
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(false, numcompare("I", "I"));
        assert_eq!(true, numcompare("I", "II"));
        assert_eq!(false, numcompare("II", "I"));
        assert_eq!(false, numcompare("V", "IIII"));
        assert_eq!(true, numcompare("MDCLXV", "MDCLXVI"));
        assert_eq!(false, numcompare("MM", "MDCCCCLXXXXVIIII"));
    }
}
