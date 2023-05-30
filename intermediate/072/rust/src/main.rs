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

#[derive(Debug, PartialEq, Eq, Clone)]
enum Color {
    Empty,
    Red,
    Blue,
    Purple,
}

impl Default for Color {
    fn default() -> Self {
        Color::Empty
    }
}

impl Color {
    fn to_char(&self) -> char {
        match self {
            Color::Empty => ' ',
            Color::Red => 'R',
            Color::Blue => 'B',
            Color::Purple => 'P',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Xray(Vec<Vec<Color>>);

impl Xray {
    fn new() -> Self {
        Xray(Vec::new())
    }

    fn add_sheet(&mut self, top_left: Coordinate, bottom_right: Coordinate, color: Color) {
        if Color::Empty == color || Color::Purple == color {
            return;
        }
        if bottom_right.y > self.0.len() {
            self.0.resize(bottom_right.y + 1, Vec::new());
        }
        for row in 0..=bottom_right.y {
            if bottom_right.x > self.0[row].len() {
                self.0[row].resize(bottom_right.x + 1, Color::default());
            }
        }
        for row in top_left.y..=bottom_right.y {
            for column in top_left.x..=bottom_right.x {
                match self.0[row][column] {
                    Color::Empty => self.0[row][column] = color.clone(),
                    Color::Red => {
                        if Color::Blue == color {
                            self.0[row][column] = Color::Purple;
                        }
                    }
                    Color::Blue => {
                        if Color::Red == color {
                            self.0[row][column] = Color::Purple;
                        }
                    }
                    Color::Purple => {}
                }
            }
        }
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
    fn test_color_to_char() {
        assert_eq!(' ', Color::Empty.to_char());
        assert_eq!('R', Color::Red.to_char());
        assert_eq!('B', Color::Blue.to_char());
        assert_eq!('P', Color::Purple.to_char());
    }

    #[test]
    fn test_add_sheet() {
        let mut xray = Xray::new();
        assert_eq!(Vec::<Vec<Color>>::new(), xray.0);
        xray.add_sheet(
            Coordinate { x: 1, y: 1 },
            Coordinate { x: 2, y: 2 },
            Color::Red,
        );
        assert_eq!(
            vec![
                vec![Color::Empty, Color::Empty, Color::Empty],
                vec![Color::Empty, Color::Red, Color::Red],
                vec![Color::Empty, Color::Red, Color::Red]
            ],
            xray.0
        );
        xray.add_sheet(
            Coordinate { x: 0, y: 0 },
            Coordinate { x: 1, y: 1 },
            Color::Blue,
        );
        assert_eq!(
            vec![
                vec![Color::Blue, Color::Blue, Color::Empty],
                vec![Color::Blue, Color::Purple, Color::Red],
                vec![Color::Empty, Color::Red, Color::Red]
            ],
            xray.0
        );
        xray.add_sheet(
            Coordinate { x: 1, y: 1 },
            Coordinate { x: 1, y: 1 },
            Color::Red,
        );
        assert_eq!(
            vec![
                vec![Color::Blue, Color::Blue, Color::Empty],
                vec![Color::Blue, Color::Purple, Color::Red],
                vec![Color::Empty, Color::Red, Color::Red]
            ],
            xray.0
        );
        xray.add_sheet(
            Coordinate { x: 1, y: 1 },
            Coordinate { x: 1, y: 1 },
            Color::Empty,
        );
        assert_eq!(
            vec![
                vec![Color::Blue, Color::Blue, Color::Empty],
                vec![Color::Blue, Color::Purple, Color::Red],
                vec![Color::Empty, Color::Red, Color::Red]
            ],
            xray.0
        );
        xray.add_sheet(
            Coordinate { x: 0, y: 0 },
            Coordinate { x: 0, y: 0 },
            Color::Red,
        );
        assert_eq!(
            vec![
                vec![Color::Purple, Color::Blue, Color::Empty],
                vec![Color::Blue, Color::Purple, Color::Red],
                vec![Color::Empty, Color::Red, Color::Red]
            ],
            xray.0
        );
    }
}
