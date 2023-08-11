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
    In,
    Out,
}

#[derive(Debug, PartialEq, Clone)]
struct FootTraffic {
    direction: Direction,
    timestamp: usize,
    person: usize,
    room: usize,
}

impl FootTraffic {
    fn new(direction: Direction, timestamp: usize, person: usize, room: usize) -> Self {
        Self {
            direction,
            timestamp,
            person,
            room,
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
    fn foottraffic_can_create_new_instance() {
        assert_eq!(
            FootTraffic::new(Direction::In, 1, 2, 3),
            FootTraffic {
                direction: Direction::In,
                timestamp: 1,
                person: 2,
                room: 3
            }
        )
    }
}
