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

const CHROMATIC_SCALE: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

#[derive(Debug, PartialEq)]
enum Chord {
    Major,
    Minor,
    DominantSeventh,
    MajorSeventh,
    MinorSeventh,
}

impl Chord {
    fn get_tones(&self) -> Vec<usize> {
        todo!()
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chord_get_tones() {
        assert_eq!(vec![0, 4, 7], Chord::Major.get_tones());
        assert_eq!(vec![0, 3, 7], Chord::Minor.get_tones());
        assert_eq!(vec![0, 4, 7, 10], Chord::DominantSeventh.get_tones());
        assert_eq!(vec![0, 4, 7, 11], Chord::MajorSeventh.get_tones());
        assert_eq!(vec![0, 3, 7, 10], Chord::MinorSeventh.get_tones());
    }
}
