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

use itertools::Itertools;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn count_exercise_sets(input: Vec<u32>) -> u32 {
    let mut count = 0;
    for mut set in input.iter().powerset().collect::<Vec<_>>() {
        if set.len() < 3 {
            continue;
        }
        set.sort();
        let last = set.pop().unwrap();
        if set.into_iter().sum::<u32>() == *last {
            count += 1;
        }
    }
    count
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_exercise_sets() {
        assert_eq!(
            179,
            count_exercise_sets(vec![
                3, 4, 9, 14, 15, 19, 28, 37, 47, 50, 54, 56, 59, 61, 70, 73, 78, 81, 92, 95, 97, 99
            ])
        );
    }
}
