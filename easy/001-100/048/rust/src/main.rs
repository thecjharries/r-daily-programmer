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

fn main() {
    println!("rad");
}

fn move_even_before_odd(input: &mut Vec<u32>) {
    let mut right_index = 0;
    let mut left_index = input.len() - 1;
    while right_index < left_index {
        if 0 == input[right_index] % 2 {
            right_index += 1;
        }
        if 1 == input[left_index] % 2 {
            left_index -= 1;
        }
        if right_index < left_index {
            input.swap(right_index, left_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_even_before_odd() {
        let mut input: Vec<u32> = vec![1, 2, 3, 4, 5];
        move_even_before_odd(&mut input);
        assert_eq!(
            vec![4, 2, 3, 1, 5],
            input
        );
        move_even_before_odd(&mut input);
        assert_eq!(
            vec![4, 2, 3, 1, 5],
            input
        );
    }
}
