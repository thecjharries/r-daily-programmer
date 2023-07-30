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

fn calculate_cannon_timing(
    sim_time: f32,
    shell_time: f32,
    propellant_time: f32,
    fire_time: f32,
) -> u32 {
    (1.0 + (sim_time - shell_time.max(propellant_time))
        / (shell_time.max(propellant_time).max(fire_time)))
    .floor() as u32
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_cannon_timing() {
        assert_eq!(2, calculate_cannon_timing(5.0, 2.00, 2.00, 1.00));
        assert_eq!(20, calculate_cannon_timing(99.99, 1.23, 3.21, 5.01));
    }
}
