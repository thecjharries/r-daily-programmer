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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_bounding_rectangle(circles: Vec<(f32, f32, f32)>) -> (f32, f32, f32, f32) {
    let mut min_x = std::f32::MAX;
    let mut min_y = std::f32::MAX;
    let mut max_x = std::f32::MIN;
    let mut max_y = std::f32::MIN;
    for (x, y, radius) in circles {
        min_x = min_x.min(x - radius);
        min_y = min_y.min(y - radius);
        max_x = max_x.max(x + radius);
        max_y = max_y.max(y + radius);
    }
    (min_x, min_y, max_x, max_y)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            (-3.0, -5.0, 6.0, 3.0),
            find_bounding_rectangle(vec![
                (1.0, 1.0, 2.0),
                (2.0, 2.0, 0.5),
                (-1.0, -3.0, 2.0),
                (5.0, 2.0, 1.0)
            ])
        );
    }
}
