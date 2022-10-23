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

use rand::prelude::*;
use rand::Rng;
use rand_pcg::Pcg64;
use std::collections::HashMap;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn generate_lotto_list<R: Rng>(names: Vec<String>, rng: &mut R) -> HashMap<String, Vec<String>> {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_lotto_list() {
        let names = vec![
            "Rebbeca Gann".to_string(),
            "Latosha Caraveo".to_string(),
            "Jim Bench".to_string(),
            "Carmelina Biles".to_string(),
            "Oda Wilhite".to_string(),
            "Arletha Eason".to_string(),
        ];
        let output: HashMap<String, Vec<String>> = HashMap::from_iter(vec![
            (
                "Latosha Caraveo".to_string(),
                vec![
                    "Jim Bench".to_string(),
                    "Carmelina Biles".to_string(),
                    "Arletha Eason".to_string(),
                ],
            ),
            (
                "Carmelina Biles".to_string(),
                vec![
                    "Oda Wilhite".to_string(),
                    "Arletha Eason".to_string(),
                    "Rebbeca Gann".to_string(),
                ],
            ),
            (
                "Oda Wilhite".to_string(),
                vec![
                    "Latosha Caraveo".to_string(),
                    "Jim Bench".to_string(),
                    "Rebbeca Gann".to_string(),
                ],
            ),
            (
                "Rebbeca Gann".to_string(),
                vec![
                    "Jim Bench".to_string(),
                    "Oda Wilhite".to_string(),
                    "Carmelina Biles".to_string(),
                ],
            ),
            (
                "Arletha Eason".to_string(),
                vec![
                    "Jim Bench".to_string(),
                    "Oda Wilhite".to_string(),
                    "Latosha Caraveo".to_string(),
                ],
            ),
            (
                "Jim Bench".to_string(),
                vec![
                    "Latosha Caraveo".to_string(),
                    "Arletha Eason".to_string(),
                    "Rebbeca Gann".to_string(),
                ],
            ),
        ]);
        let mut rng = Pcg64::seed_from_u64(0);
        assert_eq!(output, generate_lotto_list(names, 3, &mut rng));
    }
}
