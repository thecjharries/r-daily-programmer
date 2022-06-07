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

fn iterate_game_of_life(width: i32, height: i32, board: &str, iterations: i32) -> String {
    let mut current_board = board.replace("\n", "").to_string();
    let mut new_board = String::new();
    for _ in 0..iterations {
        let board_chars: Vec<char> = current_board.chars().collect::<Vec<char>>();
        for y in 0..height {
            for x in 0..width {
                let mut neighbours = 0;
                for y_ in vec![(y - 1 + height) % height, y, (y + 1 + height) % height] {
                    for x_ in vec![(x - 1 + width) % width, x, (x + 1 + width) % width] {
                        if x == x_ && y == y_ {
                            continue;
                        }
                        if '#' == board_chars[(y_ * width + x_) as usize] {
                            neighbours += 1;
                        }
                    }
                }
                if 3 == neighbours && '.' == board_chars[(y * width + x) as usize] {
                    new_board.push('#');
                } else if (2 > neighbours || 3 < neighbours)
                    && '#' == board_chars[(y * width + x) as usize]
                {
                    new_board.push('.');
                } else {
                    new_board.push(board_chars[(y * width + x) as usize]);
                }
            }
        }
        current_board = new_board.clone();
        new_board.clear();
    }
    current_board
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!("...........................................#..........##.......##...................................", iterate_game_of_life(10, 10,"......................#..........#.......###........................................................", 7));
    }
}
