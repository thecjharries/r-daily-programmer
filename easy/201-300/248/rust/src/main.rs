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

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Grid {
    width: u32,
    height: u32,
    points: Vec<Color>,
}

impl Grid {
    fn new(width: u32, height: u32) -> Grid {
        Grid {
            width,
            height,
            points: vec![
                Color {
                    red: 0,
                    green: 0,
                    blue: 0,
                };
                (width * height) as usize
            ],
        }
    }
    fn point(&self, color: Color, x: u32, y: u32) -> Point {
        self.points[(x + y * self.width) as usize] = color;
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
