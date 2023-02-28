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

fn eng_to_dec(eng: &str) -> u8 {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eng_to_dec() {
        assert_eq!(0, eng_to_dec("zero"));
        assert_eq!(1, eng_to_dec("one"));
        assert_eq!(2, eng_to_dec("two"));
        assert_eq!(3, eng_to_dec("three"));
        assert_eq!(4, eng_to_dec("four"));
        assert_eq!(5, eng_to_dec("five"));
        assert_eq!(6, eng_to_dec("six"));
        assert_eq!(7, eng_to_dec("seven"));
        assert_eq!(8, eng_to_dec("eight"));
        assert_eq!(9, eng_to_dec("nine"));
    }
}
