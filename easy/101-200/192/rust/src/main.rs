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

fn carry_add(numbers: Vec<i32>) -> Vec<Vec<i32>> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carry_add() {
        let mut input = vec![23, 9, 66];
        let mut output = vec![vec![2, 3], vec![0, 9], vec![6, 6], vec![9, 8], vec![1, 0]];
        assert_eq!(carry_add(input), output);
        input = vec![559, 447];
        output = vec![
            vec![5, 5, 9],
            vec![4, 4, 7],
            vec![1, 0, 0, 6],
            vec![1, 1, 0],
        ];
        assert_eq!(carry_add(input), output);
    }
}
