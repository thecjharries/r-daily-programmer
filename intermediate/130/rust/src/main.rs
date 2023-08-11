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

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

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

    fn random_visitor(
        max_visitor: usize,
        max_room: usize,
        earliest: usize,
        latest: usize,
        rng: Pcg64,
    ) -> (Self, Self) {
        todo!()
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

    #[test]
    fn foottraffic_random_visitor_creates_a_random_instance() {
        let (visitor_in, visitor_out) =
            FootTraffic::random_visitor(10, 10, 0, 100, Pcg64::seed_from_u64(0));
        assert_eq!(Direction::In, visitor_in.direction,);
        assert_eq!(Direction::Out, visitor_out.direction,);
        assert_eq!(visitor_in.room, visitor_out.room,);
        assert_eq!(visitor_in.person, visitor_out.person,);
        assert!(visitor_in.timestamp < visitor_out.timestamp);
    }
}
