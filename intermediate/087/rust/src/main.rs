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

use std::str::FromStr;

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

impl FromStr for Chord {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "" => Ok(Chord::Major),
            "m" => Ok(Chord::Minor),
            "7" => Ok(Chord::DominantSeventh),
            "maj7" => Ok(Chord::MajorSeventh),
            "m7" => Ok(Chord::MinorSeventh),
            _ => Err(()),
        }
    }
}

impl Chord {
    fn get_tones(&self) -> Vec<usize> {
        match self {
            Chord::Major => vec![0, 4, 7],
            Chord::Minor => vec![0, 3, 7],
            Chord::DominantSeventh => vec![0, 4, 7, 10],
            Chord::MajorSeventh => vec![0, 4, 7, 11],
            Chord::MinorSeventh => vec![0, 3, 7, 10],
        }
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn generate_chord(input: &str) -> Vec<String> {
    todo!()
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

    #[test]
    fn test_chord_fromstr() {
        assert_eq!(Ok(Chord::Major), Chord::from_str(""));
        assert_eq!(Ok(Chord::Minor), Chord::from_str("m"));
        assert_eq!(Ok(Chord::DominantSeventh), Chord::from_str("7"));
        assert_eq!(Ok(Chord::MajorSeventh), Chord::from_str("maj7"));
        assert_eq!(Ok(Chord::MinorSeventh), Chord::from_str("m7"));
        assert_eq!(Err(()), Chord::from_str("asdf"));
    }

    #[test]
    fn test_generate_chord() {
        assert_eq!(vec!["F", "A", "C"], generate_chord("F"));
        assert_eq!(vec!["F", "A", "C", "D#"], generate_chord("F7"));
    }
}
