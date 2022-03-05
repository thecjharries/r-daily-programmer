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

fn convert_to_us_currency(amount: f64, denominations: Vec<f64>) -> Vec<i64> {
    let mut result = vec![0; denominations.len()];
    let mut remainder = amount;
    for (index, denomination) in denominations.iter().enumerate() {
        let count = (remainder / denomination).floor();
        remainder -= count * denomination;
        result[index] = count as i64;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_us_currency() {
        let denominations = vec![100.00, 20.00, 10.00, 5.00, 1.00, 0.25, 0.10, 0.05, 0.01];
        assert_eq!(
            convert_to_us_currency(12.33, denominations),
            vec![0, 0, 1, 0, 2, 1, 0, 1, 3]
        );
    }
}
