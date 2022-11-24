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

fn compute_months_to_take_over(
    initial_male: u128,
    initial_female: u128,
    world_domination: u128,
) -> usize {
    let mut males: Vec<u128> = vec![0; 96];
    males[2] = initial_male;
    let mut females: Vec<u128> = vec![0; 96];
    females[2] = initial_female;
    let mut months_needed: usize = 0;
    while world_domination > males.iter().sum::<u128>() + females.iter().sum::<u128>() {
        months_needed += 1;
        let new_males = females[4..].iter().map(|&x| x * 5).sum();
        let new_females = females[4..].iter().map(|&x| x * 9).sum();
        males.rotate_right(1);
        males[0] = new_males;
        females.rotate_right(1);
        females[0] = new_females;
    }
    months_needed
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_months_to_take_over() {
        assert_eq!(32, compute_months_to_take_over(2, 4, 1000000000));
        assert_eq!(36, compute_months_to_take_over(2, 4, 15000000000));
    }
}
