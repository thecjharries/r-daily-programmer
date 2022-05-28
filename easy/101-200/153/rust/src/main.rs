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

use cached::proc_macro::cached;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cached]
fn factorial(n: u64) -> u64 {
    if n < 2 {
        return 1;
    }
    n * factorial(n - 1)
}

fn generate_pascals_pyramid(layer: u64) -> Vec<Vec<u64>> {
    let mut result = Vec::new();
    for row in 0..=layer {
        let mut result_row: Vec<u64> = Vec::new();
        for a in (0..=layer - row).rev() {
            let b = layer - row - a;
            result_row.push(factorial(layer) / (factorial(row) * factorial(a) * factorial(b)));
        }
        println!("{:?}", result_row);
        result.push(result_row)
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(1, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(2, factorial(2));
        assert_eq!(120, factorial(5))
    }

    #[test]
    fn test_generate_pascals_pyramid() {
        let result = generate_pascals_pyramid(14);
        let last_row = result.last().unwrap();
        assert_eq!(
            vec![1, 14, 91, 364, 1001, 2002, 3003, 3432, 3003, 2002, 1001, 364, 91, 14, 1],
            last_row
        );
    }
}
