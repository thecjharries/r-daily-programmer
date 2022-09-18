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

fn calculate_single_attack_kill_probability(dice_sides: usize, health: u32) -> f32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_single_attack_kill_probability() {
        assert_eq!(1.0, calculate_single_attack_kill_probability(4, 1));
        assert_eq!(0.25, calculate_single_attack_kill_probability(4, 4));
        assert_eq!(0.25, calculate_single_attack_kill_probability(4, 5));
        assert_eq!(0.1875, calculate_single_attack_kill_probability(4, 6));
        assert_eq!(1.0, calculate_single_attack_kill_probability(1, 10));
        assert_eq!(0.0001, calculate_single_attack_kill_probability(100, 200));
        assert_eq!(0.009765625, calculate_single_attack_kill_probability(8, 20));
    }
}
