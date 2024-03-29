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

fn garland(word: &str) -> u32 {
    let mut degree: u32 = 0;
    for index in 0..word.len() {
        if word[..index] == word[word.len() - index..] {
            degree = index as u32
        }
    }
    degree
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_garland() {
        assert_eq!(0, garland("programmer"));
        assert_eq!(1, garland("ceramic"));
        assert_eq!(2, garland("onion"));
        assert_eq!(4, garland("alfalfa"));
    }
}
