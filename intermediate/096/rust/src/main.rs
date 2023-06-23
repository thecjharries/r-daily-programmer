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

// I do not enjoy problems that are just a bunch of regex only
fn parse_english_int(input: &str) -> u32 {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(1, parse_english_int("one").unwrap());
        assert_eq!(2, parse_english_int("two").unwrap());
        assert_eq!(3, parse_english_int("three").unwrap());
        assert_eq!(4, parse_english_int("four").unwrap());
        assert_eq!(5, parse_english_int("five").unwrap());
        assert_eq!(6, parse_english_int("six").unwrap());
        assert_eq!(7, parse_english_int("seven").unwrap());
        assert_eq!(8, parse_english_int("eight").unwrap());
        assert_eq!(9, parse_english_int("nine").unwrap());
        assert!(parse_english_int("ten").is_err());
    }
}
