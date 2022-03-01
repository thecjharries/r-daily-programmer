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

fn build_binary_rotation_sequence(number: i64) -> Vec<i64> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_binary_rotation_sequence() {
        assert_eq!(build_binary_rotation_sequence(7), vec![7]);
        assert_eq!(build_binary_rotation_sequence(19), vec![19, 25, 28, 14, 7]);
        assert_eq!(
            build_binary_rotation_sequence(205),
            vec![205, 230, 115, 121, 124, 62, 31]
        );
        assert_eq!(
            build_binary_rotation_sequence(357),
            vec![357, 434, 217, 236, 118, 59, 61, 62, 31]
        );
    }
}
