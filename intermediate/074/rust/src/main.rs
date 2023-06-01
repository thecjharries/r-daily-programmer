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

impl Cell {
    fn new(coordinates: Coordinate) -> Self {
        Cell {
            coordinates,
            up: coordinates,
            down: coordinates,
            left: coordinates,
            right: coordinates,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct DancingLinks {
    matrix: Vec<Vec<bool>>,
    cells: Vec<Cell>,
}

impl DancingLinks {
    fn new(matrix: Vec<Vec<bool>>) -> Self {
        let mut cells: Vec<Cell> = Vec::new();
        for (y, row) in matrix.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if *value {
                    let mut cell = Cell::new(Coordinate::new(x, y));
                    let mut has_up = false;
                    for up in (0..y).rev() {
                        if matrix[up][x] {
                            cell.up = Coordinate::new(x, up);
                            cell.down = Coordinate::new(x, up);
                            has_up = true;
                            break;
                        }
                    }
                    for down in (y + 1)..matrix.len() {
                        if matrix[down][x] {
                            cell.down = Coordinate::new(x, down);
                            if !has_up {
                                cell.up = Coordinate::new(x, down);
                            }
                            break;
                        }
                    }
                    let mut has_left = false;
                    for left in (0..x).rev() {
                        if matrix[y][left] {
                            cell.left = Coordinate::new(left, y);
                            cell.right = Coordinate::new(left, y);
                            has_left = true;
                            break;
                        }
                    }
                    for right in (x + 1)..row.len() {
                        if matrix[y][right] {
                            cell.right = Coordinate::new(right, y);
                            if !has_left {
                                cell.left = Coordinate::new(right, y);
                            }
                            break;
                        }
                    }
                    cells.push(cell);
                }
            }
        }
        DancingLinks { matrix, cells }
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
    fn test_coordinate_new() {
        let coordinate = Coordinate::new(1, 2);
        assert_eq!(1, coordinate.x);
        assert_eq!(2, coordinate.y);
    }

    #[test]
    fn test_cell_new() {
        let coordinate = Coordinate::new(1, 2);
        let cell = Cell::new(coordinate);
        assert_eq!(coordinate, cell.coordinates);
        assert_eq!(coordinate, cell.up);
        assert_eq!(coordinate, cell.down);
        assert_eq!(coordinate, cell.left);
        assert_eq!(coordinate, cell.right);
    }

    #[test]
    fn test_dancinglinks_new() {
        let matrix = vec![
            vec![false, false, false],
            vec![false, true, false],
            vec![false, false, false],
        ];
        let dancing_links = DancingLinks::new(matrix.clone());
        assert_eq!(matrix, dancing_links.matrix);
        assert_eq!(vec![Cell::new(Coordinate::new(1, 1))], dancing_links.cells);
    }
}
