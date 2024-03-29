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

fn haar_transform(input: &[f64]) -> Vec<f64> {
    let mut output = Vec::new();
    let mut index = 0;
    while index < input.len() {
        output.push((input[index] + input[index + 1]) / 2.0);
        output.push((input[index] - input[index + 1]) / 2.0);
        index += 2;
    }
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haar_transform() {
        assert_eq!(
            vec![1.0, 0.0, 2.0, 0.0],
            haar_transform(&[1.0, 1.0, 2.0, 2.0])
        );
        assert_eq!(
            vec![1.0, 0.0, 2.0, 0.0, 3.0, 0.0, 4.0, 0.0],
            haar_transform(&[1.0, 1.0, 2.0, 2.0, 3.0, 3.0, 4.0, 4.0])
        );
    }
}
