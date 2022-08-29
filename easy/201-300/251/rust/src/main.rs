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

fn parse_nonogram(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    (Vec::new(), Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nonogram() {
        let (rows, cols) = parse_nonogram("    *\n   **\n  * *\n *  *\n*****");
        assert_eq!(
            vec![vec![1], vec![2], vec![1, 1], vec![1, 1], vec![5]],
            rows
        );
        assert_eq!(
            vec![vec![1], vec![2], vec![1, 1], vec![1, 1], vec![5]],
            cols
        );
    }
}
