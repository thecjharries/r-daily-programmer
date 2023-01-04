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

enum Direction {
    North,
    South,
    East,
    West,
}

struct Position {
    x: i32,
    y: i32,
}

struct Player {
    name: String,
    position: Position,
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            name,
            position: Position { x: 0, y: 0 },
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
    fn test_player_new() {
        let player = Player::new(String::from("CJ"));
        assert_eq!(String::from("CJ"), player.name);
        assert_eq!(0, player.position.x);
        assert_eq!(0, player.position.y);
    }
}
