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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_total_circle_area(center1: (f32, f32), center2: (f32, f32)) -> f32 {
    let distance = ((center1.0 - center2.0).powi(2) + (center1.1 - center2.1).powi(2)).sqrt();
    if 2.0 <= distance {
        2.0 * std::f32::consts::PI
    } else {
        2.0 * std::f32::consts::PI - 2.0 * (distance / 2.0).acos()
            + (distance / 2.0) * (4.0 - distance.powi(2)).sqrt()
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_total_circle_area() {
        assert_eq!(5.054816, find_total_circle_area((-0.5, 0.0), (0.5, 0.0)));
        assert_eq!(
            2.0 * std::f32::consts::PI,
            find_total_circle_area((0.0, 0.0), (5.0, 5.0))
        );
    }
}
