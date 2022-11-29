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

fn determine_hamming_distance(first: &str, second: &str) -> u32 {
    first
        .chars()
        .zip(second.chars())
        .filter(|(first_char, second_char)| first_char != second_char)
        .count() as u32
}

fn find_center_word(words: Vec<&str>) -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_hamming_distance() {
        assert_eq!(8, determine_hamming_distance("CTCCATCACAC", "AATATCTACAT"))
    }
}
