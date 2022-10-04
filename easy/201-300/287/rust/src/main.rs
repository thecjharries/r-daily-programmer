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

fn find_largest_digit(input: u64) -> u8 {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_largest_digit() {
        assert_eq!(4, find_largest_digit(1234));
        assert_eq!(5, find_largest_digit(3253));
        assert_eq!(9, find_largest_digit(9800));
        assert_eq!(3, find_largest_digit(3333));
        assert_eq!(2, find_largest_digit(120));
    }
}
