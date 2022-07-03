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
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut max_len = 0;
    for mut number in numbers {
        let mut number_vec: Vec<i32> = Vec::new();
        while number > 0 {
            number_vec.push(number % 10);
            number /= 10;
        }
        max_len = std::cmp::max(max_len, number_vec.len());
        result.push(number_vec);
    }
    for index in 0..result.len() {
        while result[index].len() < max_len {
            result[index].push(0);
        }
    }
    let mut sum: Vec<i32> = Vec::new();
    let mut carry: Vec<i32> = vec![0];
    let mut tens_index = 0;
    let mut number_added = true;
    while number_added {
        number_added = false;
        sum.push(carry[tens_index]);
        for number in &result {
            if number.len() > tens_index {
                number_added = true;
                sum[tens_index] += number[tens_index];
            }
        }
        if number_added {
            carry.push(sum[tens_index] / 10);
            sum[tens_index] %= 10;
            tens_index += 1;
        }
    }
    result.push(sum);
    result.push(carry);
    for index in 0..result.len() {
        result[index].reverse();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carry_add() {
        let mut input = vec![23, 9, 66];
        let mut output = vec![
            vec![2, 3],
            vec![0, 9],
            vec![6, 6],
            vec![0, 9, 8],
            vec![0, 1, 0],
        ];
        assert_eq!(carry_add(input), output);
        input = vec![559, 447];
        output = vec![
            vec![5, 5, 9],
            vec![4, 4, 7],
            vec![1, 0, 0, 6],
            vec![1, 1, 1, 0],
        ];
        assert_eq!(carry_add(input), output);
    }
}
