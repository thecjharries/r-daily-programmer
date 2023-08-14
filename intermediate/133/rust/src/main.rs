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

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(input: char) -> Option<Self> {
        match input.to_ascii_uppercase() {
            'U' => Some(Direction::Up),
            'D' => Some(Direction::Down),
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Element {
    letter: char,
    radius: usize,
    directions: Vec<Direction>,
    reacted: bool,
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
    fn direction_new_creates_all_possibilities() {
        assert_eq!(Direction::new('U'), Some(Direction::Up));
        assert_eq!(Direction::new('D'), Some(Direction::Down));
        assert_eq!(Direction::new('L'), Some(Direction::Left));
        assert_eq!(Direction::new('R'), Some(Direction::Right));
        assert_eq!(Direction::new('u'), Some(Direction::Up));
        assert_eq!(Direction::new('d'), Some(Direction::Down));
        assert_eq!(Direction::new('l'), Some(Direction::Left));
        assert_eq!(Direction::new('r'), Some(Direction::Right));
        assert_eq!(Direction::new('a'), None);
        assert_eq!(Direction::new('A'), None);
    }
}
