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

fn calculate_weight_on_planet(mass: f64, name: &str, radius: f64, average_density: f64) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_weight_on_planet() {
        assert_eq!(
            "Tantalus: 434.467",
            calculate_weight_on_planet(100.0, "Tantalus", 3104500.0, 5009.0)
        );
        assert_eq!(
            "Reach: 1059.536",
            calculate_weight_on_planet(100.0, "Reach", 7636500.0, 4966.0)
        );
        assert_eq!(
            "Circumstance: 476.441",
            calculate_weight_on_planet(100.0, "Circumstance", 4127000.0, 4132.0)
        );
        assert_eq!(
            "Tribute: 343.117",
            calculate_weight_on_planet(100.0, "Tribute", 2818000.0, 4358.0)
        );
    }
}
