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

// Tarpaulin doesn't think the return value gets covered
#[cfg(not(tarpaulin_include))]
fn rotate_2d_array_90<T>(array: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    let mut result: Vec<Vec<T>> = Vec::new();
    for _ in 0..array[0].len() {
        result.push(Vec::new() as Vec<T>);
    }
    for row_index in (0..array.len()).rev() {
        for (i, col) in array[row_index].iter().enumerate() {
            result[i].push(*col);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_2d_array_90() {
        assert_eq!(
            vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]],
            rotate_2d_array_90(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
    }
}
