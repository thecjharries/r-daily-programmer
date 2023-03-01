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

fn tpk_algorithm(input: Vec<f32>) -> Vec<f32> {
    input
        .iter()
        .map(|&value| {
            if 400.0 < value {
                f32::MAX
            } else {
                value.abs().sqrt() + 5.0 * value.powi(3)
            }
        })
        .collect()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tpk_algorithm() {
        assert_eq!(
            vec![6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0],
            tpk_algorithm(vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0])
        );
        assert_eq!(
            vec![f32::MAX, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0, 6.0],
            tpk_algorithm(vec![
                1000.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0
            ])
        );
    }
}
