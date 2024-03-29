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

fn is_jolly_jumper(sequence: Vec<i32>) -> bool {
    let mut differences = Vec::new();
    for (index, number) in sequence.iter().enumerate() {
        if index + 1 < sequence.len() {
            differences.push((number - sequence[index + 1]).abs());
        }
    }
    differences.sort();
    differences.dedup();
    differences.len() == sequence.len() - 1
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_jolly_jumper() {
        assert_eq!(false, is_jolly_jumper(vec![1, 6, -1, 8, 9, 5, 2, 7]));
        assert_eq!(true, is_jolly_jumper(vec![1, 4, 2, 3]));
        assert_eq!(false, is_jolly_jumper(vec![1, 4, 2, -1, 6]));
        assert_eq!(false, is_jolly_jumper(vec![19, 22, 24, 21]));
        assert_eq!(true, is_jolly_jumper(vec![19, 22, 24, 25]));
        assert_eq!(true, is_jolly_jumper(vec![2, -1, 0, 2]));
    }
}
