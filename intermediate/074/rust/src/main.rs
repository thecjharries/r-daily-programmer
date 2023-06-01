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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Cell {
    coordinates: Coordinate,
    up: Coordinate,
    down: Coordinate,
    left: Coordinate,
    right: Coordinate,
}

// impl Cell {
//     fn new(coordinates: Coordinate) -> Self {
//         Cell {
//             coordinates,
//             up: coordinates,
//             down: coordinates,
//             left: coordinates,
//             right: coordinates,
//         }
//     }
// }

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct DancingLinks {
    matrix: Vec<Vec<bool>>,
    cells: Vec<Cell>,
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
    fn test_coordinate_new() {
        let coordinate = Coordinate::new(1, 2);
        assert_eq!(1, coordinate.x);
        assert_eq!(2, coordinate.y);
    }
}
