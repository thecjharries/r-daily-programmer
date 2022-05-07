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

fn sum_the_digits(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut number: i32;
    match input.parse::<i32>() {
        Ok(n) => number = n,
        Err(_) => return result,
    };
    result.push(input.to_string());
    while 10 < number {
        let mut sum = 0;
        while number > 0 {
            sum += number % 10;
            number /= 10;
        }
        number = sum;
        result.push(sum.to_string());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_the_digits() {
        assert_eq!(sum_the_digits("12345"), vec!["12345", "15", "6"]);
        assert_eq!(sum_the_digits(""), Vec::new() as Vec<String>);
        assert_eq!(sum_the_digits("0"), vec!["0"]);
        assert_eq!(sum_the_digits("rad"), Vec::new() as Vec<String>);
    }
}
