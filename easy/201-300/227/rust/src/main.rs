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

use std::cmp;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_spiral_point_from_number(spiral_size: i32, number: i32) -> (i32, i32) {
    let coefficient = 1 + ((((number - 1) as f32).sqrt().floor() - 1.0) / 2.0).floor() as i32;
    (
        1 + spiral_size / 2
            + cmp::min(
                coefficient,
                cmp::max(
                    -coefficient,
                    (number - 4 * coefficient * coefficient - coefficient - 1).abs()
                        - 2 * coefficient,
                ),
            ),
        1 + spiral_size / 2
            + cmp::min(
                coefficient,
                cmp::max(
                    -coefficient,
                    (number - 4 * coefficient * coefficient + coefficient - 1).abs()
                        - 2 * coefficient,
                ),
            ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_spiral_point_from_number() {
        assert_eq!((2, 3), find_spiral_point_from_number(3, 8));
        assert_eq!((1, 1), find_spiral_point_from_number(7, 37));
        assert_eq!((6, 8), find_spiral_point_from_number(9, 47));
    }
}
