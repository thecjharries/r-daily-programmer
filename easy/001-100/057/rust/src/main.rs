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

fn find_maximum_subarray(input: Vec<i32>) -> Vec<i32> {
    let mut max_subarray = Vec::new();
    let mut current_subarray = Vec::new();
    let mut max_sum = i32::MIN;
    let mut current_sum = 0;
    for number in input {
        if current_sum <= 0 {
            current_sum = number;
            current_subarray = vec![number];
        } else {
            current_sum += number;
            current_subarray.push(number);
        }
        if current_sum >= max_sum {
            max_sum = current_sum;
            max_subarray = current_subarray.clone();
        }
    }
    max_subarray
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_maximum_subarray() {
        let mut input = vec![31, -41, 59, 26, -53, 58, 97, -93, -23, 84];
        let mut output = vec![59, 26, -53, 58, 97];
        assert_eq!(find_maximum_subarray(input), output);
        input = vec![-31, -41, -59, -26, -53, -58, -97, -93, -23, -84];
        output = vec![-23];
        assert_eq!(find_maximum_subarray(input), output);
        input = vec![31, 41, 59, 26, 53, 58, 97, 93, 23, 84];
        output = vec![31, 41, 59, 26, 53, 58, 97, 93, 23, 84];
        assert_eq!(find_maximum_subarray(input), output);
    }
}
