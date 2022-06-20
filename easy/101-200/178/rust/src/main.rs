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

use std::f32::consts::PI;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

enum Axis {
    X,
    Y,
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    fn translate(&self, x: f32, y: f32) -> Point {
        Point {
            x: self.x + x,
            y: self.y + y,
        }
    }

    fn scale(&self, x: f32, y: f32, factor: f32) -> Point {
        Point {
            x: (self.x - x) * factor + x,
            y: (self.y - y) * factor + y,
        }
    }

    fn rotate(&self, x: f32, y: f32, angle: f32) -> Point {
        Point {
            x: (self.x - x) * (-1.0 * angle).cos() - (self.y - y) * (-1.0 * angle).sin() + x,
            y: (self.x - x) * (-1.0 * angle).sin() + (self.y - y) * (-1.0 * angle).cos() + y,
        }
    }

    fn reflect(&self, axis: Axis) -> Point {
        match axis {
            Axis::X => Point {
                x: self.x * -1.0,
                y: self.y,
            },
            Axis::Y => Point {
                x: self.x,
                y: self.y * -1.0,
            },
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_new() {
        assert_eq!(Point { x: 0.0, y: 5.0 }, Point::new(0.0, 5.0));
    }

    #[test]
    fn test_point_translate() {
        assert_eq!(
            Point { x: 3.0, y: 7.0 },
            Point::new(0.0, 5.0).translate(3.0, 2.0)
        );
    }

    #[test]
    fn test_point_scale() {
        assert_eq!(
            Point { x: 2.0, y: 5.0 },
            Point::new(3.0, 7.0).scale(1.0, 3.0, 0.5)
        )
    }

    #[test]
    fn test_point_rotate() {
        assert_eq!(
            Point { x: 6.0, y: 3.0 },
            Point::new(2.0, 5.0).rotate(3.0, 2.0, PI / 2.0)
        )
    }

    #[test]
    fn test_point_reflect() {
        assert_eq!(
            Point { x: -2.0, y: 5.0 },
            Point::new(2.0, 5.0).reflect(Axis::X)
        );
        assert_eq!(
            Point { x: 2.0, y: -5.0 },
            Point::new(2.0, 5.0).reflect(Axis::Y)
        );
    }

    #[test]
    fn test_point_display() {
        assert_eq!("(2, 5)", format!("{}", Point::new(2.0, 5.0)));
    }

    #[test]
    fn test_chain() {
        assert_eq!(
            Point { x: -6.0, y: 3.0 },
            Point::new(0.0, 5.0)
                .translate(3.0, 2.0)
                .scale(1.0, 3.0, 0.5)
                .rotate(3.0, 2.0, PI / 2.0)
                .reflect(Axis::X)
        );
    }
}
