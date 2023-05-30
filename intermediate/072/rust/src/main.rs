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

    fn add_sheet(&self, top_left: Coordinate, bottom_right: Coordinate, color: Color) {
        if Color::Empty == color || Color::Purple == color {
            return;
        }
        let mut new_sheet = self.0.clone();
        for y in top_left.y..=bottom_right.y {
            for x in top_left.x..=bottom_right.x {
                match new_sheet[y][x] {
                    Color::Red => {
                        if Color::Blue == color {
                            new_sheet[y][x] = Color::Purple;
                        }
                    }
                    Color::Blue => {
                        if Color::Red == color {
                            new_sheet[y][x] = Color::Purple;
                        }
                    }
                    _ => {
                        unreachable!()
                    }
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
}
