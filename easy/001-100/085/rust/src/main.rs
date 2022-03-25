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

fn parse_matrix(input: &str) -> Vec<Vec<i64>> {
    let mut matrix = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for num in line.split_whitespace() {
            row.push(num.parse().unwrap());
        }
        matrix.push(row);
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_matrix() {
        assert_eq!(
            parse_matrix(
                "    10 5 4 20
    9 33 27 16
    11 6 55 3
"
            ),
            vec![vec![10, 5, 4, 20], vec![9, 33, 27, 16], vec![11, 6, 55, 3]]
        );
    }
}
