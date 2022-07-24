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

fn calculate_standard_deviation(input: Vec<i32>) -> f32 {
    let mut mean = 0.0;
    for i in &input {
        mean += *i as f32;
    }
    mean /= input.len() as f32;
    let mut sum_of_squares = 0.0;
    for i in &input {
        sum_of_squares += (*i as f32 - mean).powi(2);
    }
    let variance = sum_of_squares / input.len() as f32;
    (variance.sqrt() * 10000.0).round() / 10000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_standard_deviation() {
        assert_eq!(
            9.7775,
            calculate_standard_deviation(vec![5, 6, 11, 13, 19, 20, 25, 26, 28, 37])
        );
        assert_eq!(
            23.2908,
            calculate_standard_deviation(vec![
                37, 81, 86, 91, 97, 108, 109, 112, 112, 114, 115, 117, 121, 123, 141
            ])
        );
    }
}
