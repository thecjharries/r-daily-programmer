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

fn get_sliding_window_minimum(input: Vec<i32>, window_size: usize) -> Vec<i32> {
    let mut minimums = Vec::new();
    if window_size > input.len() {
        return minimums;
    }
    for window_index in 0..input.len() - window_size + 1 {
        let mut min = input[window_index];
        for i in window_index..window_index + window_size {
            if input[i] < min {
                min = input[i];
            }
        }
        minimums.push(min);
    }
    minimums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sliding_window_minimum() {
        let mut input = vec![5, 2, 8, 6, 4, 7];
        let mut output = Vec::new();
        let window_size = 20;
        assert_eq!(get_sliding_window_minimum(input, window_size), output);
        input = vec![4, 3, 2, 1, 5, 7, 6, 8, 9];
        output = vec![2, 1, 1, 1, 5, 6, 6];
        window_size = 3;
        assert_eq!(get_sliding_window_minimum(input, window_size), output);
        input = vec![5, 2, 8, 6, 4, 7];
        output = vec![2, 2, 6, 4, 4];
        window_size = 2;
        assert_eq!(get_sliding_window_minimum(input, window_size), output);
    }
}
