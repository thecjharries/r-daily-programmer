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

fn is_magic_square(input: Vec<u32>) -> bool {
    let mut first_diagonal_sum = 0;
    let mut second_diagonal_sum = 0;
    for index in 0..3 {
        let mut row_sum = 0;
        let mut column_sum = 0;
        for inner_index in 0..3 {
            row_sum += input[index * 3 + inner_index];
            column_sum += input[inner_index * 3 + index];
        }
        if 15 != row_sum || 15 != column_sum {
            return false;
        }
        first_diagonal_sum += input[index * 3 + index];
        second_diagonal_sum += input[index * 3 + 2 - index];
    }
    15 == first_diagonal_sum && 15 == second_diagonal_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_magic_square() {
        assert_eq!(true, is_magic_square(vec![8, 1, 6, 3, 5, 7, 4, 9, 2]));
        assert_eq!(true, is_magic_square(vec![2, 7, 6, 9, 5, 1, 4, 3, 8]));
        assert_eq!(false, is_magic_square(vec![3, 5, 7, 8, 1, 6, 4, 9, 2]));
        assert_eq!(false, is_magic_square(vec![8, 1, 6, 7, 5, 3, 4, 9, 2]));
    }
}
