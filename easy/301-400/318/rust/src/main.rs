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

use evalexpr::eval_int;
use itertools::Itertools;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn countdown(numbers: Vec<u32>) -> String {
    let operations = vec!["+", "-", "*", "/"];
    let mut operation_permutations = Vec::new();
    for permutation in (0..numbers.len())
        .map(|_| operations.iter())
        .multi_cartesian_product()
    {
        operation_permutations.push(permutation);
    }
    for operation_permutation in operation_permutations {
        for number_permutation in numbers.iter().permutations(numbers.len()) {
            let expression = number_permutation[0..numbers.len() - 2]
                .iter()
                .zip(operation_permutation.iter())
                .map(|(number, operator)| format!("{} {} ", number, operator))
                .join("")
                + &number_permutation[number_permutation.len() - 2].to_string();
            if let Ok(result) = eval_int(&expression) {
                if result as u32 == *number_permutation[number_permutation.len() - 1] {
                    return format!("{} = {}", expression, result);
                }
            }
        }
    }
    String::new()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        // assert_eq!(
        //     "25 - 9 * 7 * 7 + 100 - 3 = 881",
        //     countdown(vec![25, 100, 9, 7, 3, 7], 881)
        // );
        // assert_eq!(
        //     "3 + 3 * 7 + 1 * 6 - 8 = 250",
        //     countdown(vec![1, 3, 7, 6, 8, 3, 250])
        // );
        assert_eq!("5 + 5 = 10", countdown(vec![5, 5, 10]))
    }
}
