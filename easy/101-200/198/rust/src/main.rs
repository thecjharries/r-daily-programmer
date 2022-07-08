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

#[derive(Debug, PartialEq)]
enum CollisionResult {
    First,
    Second,
    Tie,
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn determine_collision_winner(first: &str, second: &str) -> CollisionResult {
    let mut first_chars = first.chars().collect::<Vec<_>>();
    let mut second_chars = second.chars().collect::<Vec<_>>();
    let mut first_index = 0;
    while first_index < first_chars.len() {
        let mut not_found = true;
        for second_index in 0..second_chars.len() {
            if first_chars[first_index] == second_chars[second_index] {
                not_found = false;
                first_chars.remove(first_index);
                second_chars.remove(second_index);
                break;
            }
        }
        if not_found {
            first_index += 1;
        }
    }
    if first_chars.len() > second_chars.len() {
        CollisionResult::First
    } else if first_chars.len() < second_chars.len() {
        CollisionResult::Second
    } else {
        CollisionResult::Tie
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            CollisionResult::First,
            determine_collision_winner("because", "cause")
        );
        assert_eq!(
            CollisionResult::Second,
            determine_collision_winner("cause", "because")
        );
        assert_eq!(
            CollisionResult::Tie,
            determine_collision_winner("hello", "below")
        );
    }
}
