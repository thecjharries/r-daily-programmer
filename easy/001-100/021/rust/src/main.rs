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

fn find_next_largest_permutation(input: u32) -> u32 {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_next_largest_permutation() {
        assert_eq!(find_next_largest_permutation(5), 5);
        assert_eq!(find_next_largest_permutation(5987643), 6345789);
        assert_eq!(find_next_largest_permutation(38276), 38627);
        assert_eq!(find_next_largest_permutation(987654321), 987654321);
        assert_eq!(find_next_largest_permutation(9376544321), 9412334567);
    }
}
