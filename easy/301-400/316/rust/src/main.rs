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

// https://old.reddit.com/r/dailyprogrammer/comments/6coqwk/20170522_challenge_316_easy_knights_metric/dhw8sdx/
fn total_knight_moves(goal_x: i32, goal_y: i32) -> i32 {
    let mut x = goal_x;
    let mut y = goal_y;
    if x > y {
        std::mem::swap(&mut x, &mut y);
    }
    let mut additional_move = 0;
    if 0 != (x + y) % 3 {
        additional_move = 1;
        if 0 == (x + 2 + y - 1) % 3 {
            x += 2;
            y -= 1;
        } else {
            x += 1;
            y -= 2;
        }
    }
    let first = (2 * y - x) / 3;
    let second = (2 * x - y) / 3 + additional_move;
    first.abs() + second.abs()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(4, total_knight_moves(3, 7));
        assert_eq!(0, total_knight_moves(0, 0));
        assert_eq!(3, total_knight_moves(1, 0));
        assert_eq!(2, total_knight_moves(-3, -3));
        assert_eq!(2, total_knight_moves(0, -2));
        assert_eq!(4, total_knight_moves(1, 1));
    }
}
