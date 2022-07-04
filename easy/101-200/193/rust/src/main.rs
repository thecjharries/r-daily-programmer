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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn calculate_from_volume(volume: f32) -> String {
    let cube_root = volume.powf(1.0 / 3.0);
    format!(
        "Cube: {cube_root}m width, {cube_root}m, high, {cube_root}m tall\nCylinder: {cube_root}m tall, Diameter of {}m\nSphere: {}m Radius\nCone: {}m tall, {}m Radius",
        2.0 *(volume / (PI * cube_root)).sqrt(),
        (3.0 *volume/PI/4.0).powf(1.0 / 3.0),
        cube_root.powi(2),
        (3.0*volume/PI/cube_root.powi(2)).sqrt(),
        cube_root = cube_root
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_from_volume() {
        assert_eq!("Cube: 3m width, 3m, high, 3m tall\nCylinder: 3m tall, Diameter of 3.3851376m\nSphere: 1.8610514m Radius\nCone: 9m tall, 1.6925688m Radius", calculate_from_volume(27.0));
    }
}
