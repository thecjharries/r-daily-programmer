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

fn get_first_n_look_and_say_iterations(iterations: usize, seed: u128) -> Vec<u128> {
    let mut sequence = vec![seed];
    while iterations > sequence.len() {
        let previous = sequence
            .last()
            .unwrap()
            .to_string()
            .chars()
            .collect::<Vec<_>>();
        let mut next = String::new();
        let mut current = previous[0];
        let mut count = 1;
        for i in 1..previous.len() {
            if previous[i] == current {
                count += 1;
            } else {
                next.push_str(&format!("{}{}", count, current));
                current = previous[i];
                count = 1;
            }
        }
        next.push_str(&format!("{}{}", count, current));
        sequence.push(next.parse::<u128>().unwrap());
    }
    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_n_look_and_say_iterations() {
        assert_eq!(
            vec![1, 11, 21, 1211, 111221, 312211, 13112221],
            get_first_n_look_and_say_iterations(7, 1)
        );
        assert_eq!(
            vec![22, 22, 22, 22, 22, 22, 22, 22],
            get_first_n_look_and_say_iterations(8, 22)
        )
    }
}
