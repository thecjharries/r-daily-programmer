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

use std::f64::consts::PI;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn compute_angles(a: f64, b: f64, c: f64) -> (f64, f64, f64) {
    (
        ((a / c).asin() * 180.0 / PI * 100.0).round() / 100.0,
        ((b / c).asin() * 180.0 / PI * 100.0).round() / 100.0,
        90.0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_angles() {
        assert_eq!((36.87, 53.13, 90.0), compute_angles(3.0, 4.0, 5.0));
    }
}
