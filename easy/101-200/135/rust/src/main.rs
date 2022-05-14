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

use lazy_static::lazy_static;
use meval::eval_str;
use rand::prelude::*;
use rand::Rng;
use rand_pcg::Pcg64;

lazy_static! {
    static ref OPERATIONS: Vec<String> = vec![
        "+".to_string(),
        "-".to_string(),
        "*".to_string(),
        "/".to_string()
    ];
}

fn main() {
    println!("rad");
}

fn build_equation<R: Rng>(min: i32, max: i32, rng: &mut R) -> String {
    let mut numbers: Vec<i32> = vec![];
    let mut operations: Vec<&String> = vec![];
    for _ in 0..4 {
        numbers.push(rng.gen_range(min..max + 1) as i32);
        operations.push(&OPERATIONS[rng.gen_range(0..OPERATIONS.len())]);
    }
    format!(
        "{} {} {} {} {} {} {}",
        numbers[0], operations[0], numbers[1], operations[1], numbers[2], operations[2], numbers[3]
    )
}

fn validate_answer(equation: String, answer: i64) -> bool {
    let result = eval_str(&equation).unwrap();
    result as i64 == answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_equation() {
        assert_eq!(
            build_equation(1, 10, &mut Pcg64::seed_from_u64(0)),
            "9 - 1 + 2 * 2"
        );
    }

    #[test]
    fn test_validate_answer() {
        assert_eq!(validate_answer("1 * 5 * 9 + 8".to_string(), 10), false);
        assert_eq!(validate_answer("1 * 5 * 9 + 8".to_string(), 53), true);
    }
}
