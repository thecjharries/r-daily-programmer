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

fn note(scale: char, solfege: &str) -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note() {
        assert_eq!("C".to_string(), note('C', "Do"));
        assert_eq!("D".to_string(), note('C', "Re"));
        assert_eq!("E".to_string(), note('C', "Mi"));
        assert_eq!("F#".to_string(), note('D', "Mi"));
        assert_eq!("D#".to_string(), note('A', "Fa"));
    }
}
