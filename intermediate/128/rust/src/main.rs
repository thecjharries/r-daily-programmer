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

#[derive(Debug, PartialEq)]
enum Direction {
    Row,
    Column,
}

#[derive(Debug, PartialEq)]
struct Roads(Vec<Vec<u8>>);

impl Roads {
    fn new(roads: Vec<Vec<u8>>) -> Self {
        Self(roads)
    }

    fn find_most_potholes(&self) -> Option<(Direction, usize)> {
        let mut result = None;
        let mut most_potholes = 0;
        for (index, row) in self.0.iter().enumerate() {
            let row_potholes = row.iter().filter(|&&x| x == 0).count();
            if row_potholes > most_potholes {
                most_potholes = row_potholes;
                result = Some((Direction::Row, index));
            }
            let column_potholes = self
                .0
                .iter()
                .map(|row| row[index])
                .filter(|&x| x == 0)
                .count();
            if column_potholes > most_potholes {
                most_potholes = column_potholes;
                result = Some((Direction::Column, index));
            }
        }
        result
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
    fn road_new_builds_a_new_struct() {
        let roads = Roads::new(vec![
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![0, 1, 0, 1, 0],
        ]);
        assert_eq!(
            roads,
            Roads(vec![
                vec![0, 1, 0, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 0, 1, 0],
            ])
        );
    }

    #[test]
    fn road_find_most_potholes_identifies_the_largest_row_or_column() {
        let roads = Roads::new(vec![
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![0, 1, 0, 1, 0],
        ]);
        assert_eq!(Some((Direction::Column, 2)), roads.find_most_potholes());
        let roads = Roads::new(vec![vec![1, 1], vec![1, 1]]);
        assert!(roads.find_most_potholes().is_none());
        let roads = Roads::new(vec![vec![0, 0], vec![0, 0]]);
        assert_eq!(Some((Direction::Row, 0)), roads.find_most_potholes());
    }
}
