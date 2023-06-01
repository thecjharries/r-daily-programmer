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

#[derive(Debug,PartialEq,Eq,Copy,Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug,PartialEq,Eq,Copy,Clone)]
struct Cell {
    up: Coordinate,
    down: Coordinate,
    left: Coordinate,
    right: Coordinate,
}

#[derive(Debug,PartialEq,Eq,Copy,Clone)]
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
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
