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

fn find_all_pairs(lists: Vec<Vec<char>>) -> Vec<Vec<char>> {}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            12,
            find_all_pairs(vec![
                vec!['0', '1'],
                vec!['A', 'B', 'C'],
                vec!['D', 'E', 'F', 'G']
            ])
            .len()
        );
        assert_eq!(
            34,
            find_all_pairs(vec![
                vec!['0', '1', '2', '3'],
                vec!['A', 'B', 'C', 'D'],
                vec!['E', 'F', 'G', 'H', 'I']
            ])
            .len()
        );
        assert_eq!(
            62,
            find_all_pairs(vec![
                vec!['0', '1', '2', '3', '4'],
                vec!['A', 'B', 'C', 'D', 'E'],
                vec!['F', 'G', 'H', 'I'],
                vec!['J', 'K', 'L']
            ])
            .len()
        );
    }
}
