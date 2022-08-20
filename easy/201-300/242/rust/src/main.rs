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

fn calculate_weeks_necessary(people: u32, starting_plants: u32) -> f64 {
    if 1 > starting_plants {
        return f64::INFINITY;
    }
    let mut weeks = 1.0;
    let mut current_plants = 0;
    let mut growing_plants = starting_plants;
    while current_plants < people {
        current_plants += growing_plants;
        growing_plants += current_plants;
        weeks += 1.0;
    }
    weeks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_weeks_necessary() {
        assert_eq!(f64::INFINITY, calculate_weeks_necessary(250, 0));
        assert_eq!(5.0, calculate_weeks_necessary(200, 15));
        assert_eq!(14.0, calculate_weeks_necessary(50000, 1));
        assert_eq!(9.0, calculate_weeks_necessary(150000, 250));
    }
}
