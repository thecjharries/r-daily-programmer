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

#[derive(Debug, PartialEq, Eq)]
struct Rectangle {
    top_left: (i32, i32),
    bottom_right: (i32, i32),
}

trait Intersects {
    fn intersects(&self, other: &Self) -> Self;
}

impl Intersects for Rectangle {
    fn intersects(&self, other: &Self) -> Self {
        let mut top_left = (-1, -1);
        let mut bottom_right = (-1, -1);
        if self.top_left.0 < other.bottom_right.0 && self.bottom_right.0 > other.top_left.0 {
            top_left.0 = self.top_left.0.max(other.top_left.0);
            top_left.1 = self.top_left.1.max(other.top_left.1);
            bottom_right.0 = self.bottom_right.0.min(other.bottom_right.0);
            bottom_right.1 = self.bottom_right.1.min(other.bottom_right.1);
        } else if other.top_left.0 < self.bottom_right.0 && other.bottom_right.0 > self.top_left.0 {
            top_left.0 = other.top_left.0.max(self.top_left.0);
            top_left.1 = other.top_left.1.max(self.top_left.1);
            bottom_right.0 = other.bottom_right.0.min(self.bottom_right.0);
            bottom_right.1 = other.bottom_right.1.min(self.bottom_right.1);
        }
        Rectangle {
            top_left,
            bottom_right,
        }
    }
}

fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersects_for_rectangle() {
        let first = Rectangle {
            top_left: (3, 3),
            bottom_right: (10, 10),
        };
        let second = Rectangle {
            top_left: (6, 6),
            bottom_right: (12, 12),
        };
        assert_eq!(
            first.intersects(&second),
            Rectangle {
                top_left: (6, 6),
                bottom_right: (10, 10),
            }
        );
        let third = Rectangle {
            top_left: (4, 4),
            bottom_right: (5, 5),
        };
        let fourth = Rectangle {
            top_left: (6, 6),
            bottom_right: (10, 10),
        };
        assert_eq!(
            third.intersects(&fourth),
            Rectangle {
                top_left: (-1, -1),
                bottom_right: (-1, -1),
            }
        );
    }
}
