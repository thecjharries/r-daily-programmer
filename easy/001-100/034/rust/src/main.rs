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

fn calculate_sum_of_squares_of_two_largest(first: i32, second: i32, third: i32) -> i32 {
    let mut numbers = vec![first, second, third];
    numbers.sort();
    numbers[1].pow(2) + numbers[2].pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sum_of_squares_of_two_largest() {
        assert_eq!(calculate_sum_of_squares_of_two_largest(1, 2, 3), 13);
    }
}
