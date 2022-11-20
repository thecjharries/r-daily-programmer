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

const CHROMATIC_SCALE: [&'static str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];
const SOLFEGE_SCALE: [&'static str; 7] = ["Do", "Re", "Mi", "Fa", "Sol", "La", "Ti"];

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn build_major_scale(root: &str) -> Vec<String> {
    todo!()
}

fn note(scale: char, solfege: &str) -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_major_scale() {
        assert_eq!(
            vec!["D", "E", "F#", "G", "A", "B", "C#"],
            build_major_scale("D")
        );
    }

    #[test]
    fn test_note() {
        assert_eq!("C".to_string(), note('C', "Do"));
        assert_eq!("D".to_string(), note('C', "Re"));
        assert_eq!("E".to_string(), note('C', "Mi"));
        assert_eq!("F#".to_string(), note('D', "Mi"));
        assert_eq!("D#".to_string(), note('A', "Fa"));
    }
}
