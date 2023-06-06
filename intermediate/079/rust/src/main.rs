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

#[derive(Debug, PartialEq)]
struct PgmFile {
    width: usize,
    height: usize,
    max_value: usize,
    pixels: Vec<Vec<usize>>,
}

impl FromStr for PgmFile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines: Vec<String> = s
            .lines()
            .map(String::from)
            .filter(|line| !line.starts_with("#"))
            .collect();
        // File spec
        lines.remove(0);
        // Dimensions
        let dimensions: Vec<usize> = lines
            .remove(0)
            .split_whitespace()
            .map(|dimension| dimension.parse().unwrap())
            .collect();
        let width = dimensions[0];
        let height = dimensions[1];
        // Max value
        let max_value = lines.remove(0).parse().unwrap();
        // Pixels
        let mut pixels: Vec<Vec<usize>> = Vec::with_capacity(height);
        for _ in 0..height {
            let row: Vec<usize> = lines
                .remove(0)
                .split_whitespace()
                .map(|pixel| pixel.parse().unwrap())
                .collect();
            pixels.push(row);
        }
        Ok(PgmFile {
            width,
            height,
            max_value,
            pixels,
        })
    }
}

impl PgmFile {
    fn print_ascii(&self, gradient: Vec<char>) -> String {
        let mut output = String::new();
        for row in self.pixels.iter() {
            for pixel in row.iter() {
                output.push(
                    gradient[((*pixel as f32 / self.max_value as f32)
                        * (gradient.len() as f32 - 1.0)) as usize],
                );
            }
            output.push('\n');
        }
        output
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
    fn test_pgmfile_fromstr() {
        // P2
        // # feep.pgm
        // 24 7
        // 15
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0
        // 0  3  3  3  3  0  0  7  7  7  7  0  0 11 11 11 11  0  0 15 15 15 15  0
        // 0  3  0  0  0  0  0  7  0  0  0  0  0 11  0  0  0  0  0 15  0  0 15  0
        // 0  3  3  3  0  0  0  7  7  7  0  0  0 11 11 11  0  0  0 15 15 15 15  0
        // 0  3  0  0  0  0  0  7  0  0  0  0  0 11  0  0  0  0  0 15  0  0  0  0
        // 0  3  0  0  0  0  0  7  7  7  7  0  0 11 11 11 11  0  0 15  0  0  0  0
        // 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0
        let pgm_file = PgmFile {
            width: 24,
            height: 7,
            max_value: 15,
            pixels: vec![
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                vec![
                    0, 3, 3, 3, 3, 0, 0, 7, 7, 7, 7, 0, 0, 11, 11, 11, 11, 0, 0, 15, 15, 15, 15, 0,
                ],
                vec![
                    0, 3, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 15, 0, 0, 15, 0,
                ],
                vec![
                    0, 3, 3, 3, 0, 0, 0, 7, 7, 7, 0, 0, 0, 11, 11, 11, 0, 0, 0, 15, 15, 15, 15, 0,
                ],
                vec![
                    0, 3, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0,
                ],
                vec![
                    0, 3, 0, 0, 0, 0, 0, 7, 7, 7, 7, 0, 0, 11, 11, 11, 11, 0, 0, 15, 0, 0, 0, 0,
                ],
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            ],
        };
        assert_eq!(
            pgm_file,
            PgmFile::from_str(std::include_str!("../../feep.pgm")).unwrap()
        );
    }

    #[test]
    fn test_pgmfile_print_ascii() {
        let output = "                        \n ....  ;;;;  ====  #### \n .     ;     =     #  # \n ...   ;;;   ===   #### \n .     ;     =     #    \n .     ;;;;  ====  #    \n                        \n";
        let pgm_file = PgmFile::from_str(std::include_str!("../../feep.pgm")).unwrap();
        assert_eq!(output, pgm_file.print_ascii(" .:;+=%$#".chars().collect()));
    }
}
