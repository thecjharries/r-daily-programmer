// Copyright 2023 CJ Harries
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

fn calculate_sum(min: u64, max: u64) -> u64 {
    max * (max + 1) / 2 - (min - 1) * min / 2
}

fn calculate_sum_of_squares(min: u64, max: u64) -> u64 {
    max * (max + 1) * (2 * max + 1) / 6 - (min - 1) * min * (2 * min - 1) / 6
}

fn find_missing_two_integers(min: u64, max: u64, given: Vec<u64>) -> (u64, u64) {
    let mut sum = calculate_sum(min, max);
    let mut sum_of_squares = calculate_sum_of_squares(min, max);
    for item in given {
        sum -= item;
        sum_of_squares -= item * item;
    }
    let a = (sum - ((2 * sum_of_squares - sum * sum) as f64).sqrt() as u64) / 2;
    let b = (sum + ((2 * sum_of_squares - sum * sum) as f64).sqrt() as u64) / 2;
    (a, b)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sum() {
        assert_eq!(5050, calculate_sum(1, 100));
    }

    #[test]
    fn test_calculate_sum_of_squares() {
        assert_eq!(338350, calculate_sum_of_squares(1, 100));
    }

    #[test]
    fn test_find_missing_two_integers() {
        assert_eq!((3, 4), find_missing_two_integers(1, 5, vec![1, 2, 5]));
    }
}
