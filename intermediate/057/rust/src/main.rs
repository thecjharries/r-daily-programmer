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

#[derive(Debug, PartialEq, Eq)]
struct InvertedMatchTable {
    characters: Vec<char>,
    coordinates: Vec<Vec<u8>>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInvertedMatchTableError;

impl FromStr for InvertedMatchTable {
    type Err = ParseInvertedMatchTableError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut characters = Vec::new();
        let mut coordinates = Vec::new();
        for (row_index, row) in s.split('\n').enumerate() {
            if 0 == row_index {
                characters = row.chars().filter(|c| c.is_alphabetic()).collect();
            } else {
                coordinates.push(
                    row.chars()
                        .filter(|c| c.is_numeric())
                        .map(|character| character.to_digit(10).unwrap() as u8)
                        .collect(),
                );
            }
        }
        Ok(InvertedMatchTable {
            characters,
            coordinates,
        })
    }
}

impl InvertedMatchTable {
    fn find_inverted_matches(&self) -> Vec<((char, char), (char, char))> {
        let mut matches = Vec::new();
        for column in 0..self.coordinates.len() {
            for row in 0..column {
                if 1 == self.coordinates[row][column] && 1 == self.coordinates[column][row] {
                    matches.push((
                        (self.characters[row], self.characters[column]),
                        (self.characters[column], self.characters[row]),
                    ));
                }
            }
        }
        matches
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
    fn test_invertedmatchtable_from_str() {
        //  ABC
        // A010
        // B111
        // C011
        let result = InvertedMatchTable {
            characters: vec!['A', 'B', 'C'],
            coordinates: vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 1]],
        };
        assert_eq!(
            result,
            InvertedMatchTable::from_str(" ABC\nA010\nB111\nC011").unwrap()
        );
        //  ABCDEFGHIJKLMNOPQRST
        // A11110101111011100010
        // B10010010000010001100
        // C01101110010001000000
        // D10110011001011101100
        // E10100100011110110100
        // F01111011000111010010
        // G00011110001011001110
        // H01111000010001001000
        // I01101110010110010011
        // J00101000100010011110
        // K10101001100001100000
        // L01011010011101100110
        // M10110110010101000100
        // N10001111101111110010
        // O11011010010111100110
        // P01000110111101101000
        // Q10011001100010100000
        // R11101011100110110110
        // S00001100000110010101
        // T01000110011100101011
        let big_table = InvertedMatchTable::from_str(" ABCDEFGHIJKLMNOPQRST\nA11110101111011100010\nB10010010000010001100\nC01101110010001000000\nD10110011001011101100\nE10100100011110110100\nF01111011000111010010\nG00011110001011001110\nH01111000010001001000\nI01101110010110010011\nJ00101000100010011110\nK10101001100001100000\nL01011010011101100110\nM10110110010101000100\nN10001111101111110010\nO11011010010111100110\nP01000110111101101000\nQ10011001100010100000\nR11101011100110110110\nS00001100000110010101\nT01000110011100101011").unwrap();
        assert_eq!(
            vec![
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T'
            ],
            big_table.characters
        );
        assert_eq!(
            vec![1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0],
            big_table.coordinates[0]
        );
    }

    #[test]
    fn test_invertedmatchtable_find_inverted_matches() {
        let table = InvertedMatchTable::from_str(" ABC\nA010\nB111\nC011").unwrap();
        assert_eq!(
            vec![(('A', 'B'), ('B', 'A')), (('B', 'C'), ('C', 'B'))],
            table.find_inverted_matches()
        );
        let big_table = InvertedMatchTable::from_str(" ABCDEFGHIJKLMNOPQRST\nA11110101111011100010\nB10010010000010001100\nC01101110010001000000\nD10110011001011101100\nE10100100011110110100\nF01111011000111010010\nG00011110001011001110\nH01111000010001001000\nI01101110010110010011\nJ00101000100010011110\nK10101001100001100000\nL01011010011101100110\nM10110110010101000100\nN10001111101111110010\nO11011010010111100110\nP01000110111101101000\nQ10011001100010100000\nR11101011100110110110\nS00001100000110010101\nT01000110011100101011").unwrap();
        assert_eq!(47, big_table.find_inverted_matches().len());
    }
}
