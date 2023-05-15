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

use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct InvertedMatchTable {
    rows: HashMap<char, HashMap<char, u8>>,
    columns: HashMap<char, HashMap<char, u8>>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInvertedMatchTableError;

impl FromStr for InvertedMatchTable {
    type Err = ParseInvertedMatchTableError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = HashMap::new();
        let mut columns = HashMap::new();
        let mut lines = s.lines();
        let header = lines.next().unwrap();
        let header_chars: Vec<char> = header.chars().collect();
        for line in lines {
            let line_chars: Vec<char> = line.chars().collect();
            let row_char = line_chars[0];
            let mut row = HashMap::new();
            for (index, column_char) in line_chars.iter().enumerate().skip(1) {
                let header_char = header_chars[index];
                row.insert(header_char, *column_char as u8 - 48);
                if !columns.contains_key(&header_char) {
                    columns.insert(header_char, HashMap::new());
                }
                columns
                    .get_mut(&header_char)
                    .unwrap()
                    .insert(row_char, *column_char as u8 - 48);
            }
            rows.insert(row_char, row);
        }
        Ok(InvertedMatchTable { rows, columns })
    }
}

impl InvertedMatchTable {
    fn find_inverted_matches(&self) -> Vec<((char, char), (char, char))> {
        let mut matches = Vec::new();
        let mut characters = self.rows.keys().collect::<Vec<&char>>();
        characters.sort();
        for (row_char, index) in characters.iter().enumerate() {
            for column_char in characters.iter().skip(row_char + 1) {
                let row_val = self.rows.get(*index).unwrap().get(*column_char).unwrap();
                let column_val = self.columns.get(*column_char).unwrap().get(*index).unwrap();
                if 1 == *row_val && 1 == *column_val {
                    matches.push(((**index, **column_char), (**column_char, **index)));
                }
            }
        }
        matches.sort();
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
            rows: HashMap::from_iter(vec![
                ('A', HashMap::from_iter(vec![('A', 0), ('B', 1), ('C', 0)])),
                ('B', HashMap::from_iter(vec![('A', 1), ('B', 1), ('C', 1)])),
                ('C', HashMap::from_iter(vec![('A', 0), ('B', 1), ('C', 1)])),
            ]),
            columns: HashMap::from_iter(vec![
                ('A', HashMap::from_iter(vec![('A', 0), ('B', 1), ('C', 0)])),
                ('B', HashMap::from_iter(vec![('A', 1), ('B', 1), ('C', 1)])),
                ('C', HashMap::from_iter(vec![('A', 0), ('B', 1), ('C', 1)])),
            ]),
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
        let a_row = big_table.rows.get(&'A').unwrap();
        assert_eq!(1, *a_row.get(&'A').unwrap());
        assert_eq!(1, *a_row.get(&'B').unwrap());
        assert_eq!(1, *a_row.get(&'C').unwrap());
        assert_eq!(1, *a_row.get(&'D').unwrap());
        assert_eq!(0, *a_row.get(&'E').unwrap());
        assert_eq!(1, *a_row.get(&'F').unwrap());
        assert_eq!(0, *a_row.get(&'G').unwrap());
        assert_eq!(1, *a_row.get(&'H').unwrap());
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
