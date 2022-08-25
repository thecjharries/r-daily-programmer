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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn build_ordering<R: Rng>(people: Vec<Vec<String>>, rng: &mut R) -> Vec<(usize, usize)> {
    Vec::new()
}

fn build_secret_santa_list<R: Rng>(people: Vec<Vec<String>>, rng: &mut R) -> Vec<String> {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_ordering() {
        assert_eq!(
            vec![],
            build_ordering(
                vec![
                    vec!["Jeff".to_string(), "Jerry".to_string()],
                    vec!["Joe".to_string()],
                    vec!["Johnson".to_string()]
                ],
                &mut Pcg64::seed_from_u64(0)
            )
        )
    }

    #[test]
    fn test_build_secret_santa_list() {
        assert_eq!(
            vec![],
            build_secret_santa_list(
                vec![
                    vec!["Jeff".to_string(), "Jerry".to_string()],
                    vec!["Joe".to_string()],
                    vec!["Johnson".to_string()]
                ],
                &mut Pcg64::seed_from_u64(0)
            )
        )
    }
}
