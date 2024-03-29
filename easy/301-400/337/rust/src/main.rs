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

fn problem1() -> f32 {
    360.0 / std::f32::consts::PI
}

fn problem2(distance_a: f32, distance_b: f32, river_length: f32) -> f32 {
    distance_a * river_length / (distance_a + distance_b)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        assert_eq!(114.59155, problem1());
    }

    #[test]
    fn test_problem2() {
        assert_eq!(40.0, problem2(20.0, 30.0, 100.0));
    }
}
