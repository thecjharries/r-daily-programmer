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

fn quicksort<T: Copy + PartialEq + PartialOrd>(input: Vec<T>) -> Vec<T> {
    if 1 >= input.len() {
        return input;
    }
    let pivot_index = input.len() / 2;
    let mut smaller: Vec<T> = Vec::new();
    let mut larger: Vec<T> = Vec::new();
    for i in 0..input.len() {
        if i != pivot_index {
            if input[i] < input[pivot_index] {
                smaller.push(input[i]);
            } else {
                larger.push(input[i]);
            }
        }
    }
    let mut result: Vec<T> = Vec::new();
    result.append(&mut quicksort(smaller));
    result.push(input[pivot_index]);
    result.append(&mut quicksort(larger));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        assert_eq!(vec![1, 2, 3], quicksort(vec![3, 2, 1]));
        assert_eq!(vec![1.1, 2.2, 3.3], quicksort(vec![3.3, 2.2, 1.1]));
    }
}
