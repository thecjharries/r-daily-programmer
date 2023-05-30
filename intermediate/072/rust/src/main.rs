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
struct Xray(Vec<Vec<Color>>);

impl Xray {
    fn new() -> Self {
        Xray(Vec::new())
    }

    fn add_sheet(
        &mut self,
        top_left_x: usize,
        top_left_y: usize,
        width: usize,
        height: usize,
        color: Color,
    ) {
        if Color::Empty == color || Color::Purple == color {
            return;
        }
        if top_left_y + height > self.0.len() {
            self.0.resize(top_left_y + height, Vec::new());
        }
        for row in 0..self.0.len() {
            if top_left_x + width > self.0[row].len() {
                self.0[row].resize(top_left_x + width, Color::default());
            }
        }
        for row in top_left_y..top_left_y + height {
            for column in top_left_x..top_left_x + width {
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
        xray.add_sheet(1, 1, 2, 2, Color::Red);
        assert_eq!(
            vec![
                vec![Color::Empty, Color::Empty, Color::Empty],
                vec![Color::Empty, Color::Red, Color::Red],
                vec![Color::Empty, Color::Red, Color::Red]
            ],
            xray.0
        );
        xray.add_sheet(0, 0, 2, 2, Color::Blue);
        assert_eq!(
            vec![
                vec![Color::Blue, Color::Blue, Color::Empty],
                vec![Color::Blue, Color::Purple, Color::Red],
                vec![Color::Empty, Color::Red, Color::Red]
            ],
            xray.0
        );
        xray.add_sheet(1, 1, 1, 1, Color::Red);
        assert_eq!(
            vec![
                vec![Color::Blue, Color::Blue, Color::Empty],
                vec![Color::Blue, Color::Purple, Color::Red],
                vec![Color::Empty, Color::Red, Color::Red]
            ],
            xray.0
        );
        xray.add_sheet(1, 1, 1, 1, Color::Empty);
        assert_eq!(
            vec![
                vec![Color::Blue, Color::Blue, Color::Empty],
                vec![Color::Blue, Color::Purple, Color::Red],
                vec![Color::Empty, Color::Red, Color::Red]
            ],
            xray.0
        );
        xray.add_sheet(0, 0, 1, 1, Color::Red);
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
