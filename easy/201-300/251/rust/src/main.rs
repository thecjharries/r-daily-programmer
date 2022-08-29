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

fn parse_nonogram(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let exploded = input.replace("\n", "").chars().collect::<Vec<char>>();
    let length = (exploded.len() as f32).sqrt() as usize;
    let mut rows = Vec::new();
    for i in 0..length {
        let mut row = Vec::new();
        let mut on = 0;
        for j in 0..length {
            if '*' == exploded[i * length + j] {
                on += 1;
            } else if 0 < on {
                row.push(on);
                on = 0;
            }
        }
        if 0 < on {
            row.push(on);
        }
        rows.push(row);
    }
    let mut cols = Vec::new();
    for i in 0..length {
        let mut col = Vec::new();
        let mut on = 0;
        for j in 0..length {
            if '*' == exploded[j * length + i] {
                on += 1;
            } else if 0 < on {
                col.push(on);
                on = 0;
            }
        }
        if 0 < on {
            col.push(on);
        }
        cols.push(col);
    }
    (rows, cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nonogram() {
        let (rows, cols) = parse_nonogram("    *\n   **\n  * *\n *  *\n*****");
        assert_eq!(
            vec![vec![1], vec![2], vec![1, 1], vec![1, 1], vec![5]],
            rows
        );
        assert_eq!(
            vec![vec![1], vec![2], vec![1, 1], vec![1, 1], vec![5]],
            cols
        );
    }
}
