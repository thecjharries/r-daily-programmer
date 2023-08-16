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

fn ranked_choice_vote(candidates: Vec<&str>, votes: Vec<Vec<usize>>) -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranked_choice_vote_determines_a_clear_winner() {
        let candidates = vec!["A", "B", "C"];
        let votes = vec![
            vec![0, 1, 2],
            vec![0, 1, 2],
            vec![0, 1, 2],
            vec![0, 1, 2],
            vec![0, 1, 2],
        ];
        assert_eq!("A", ranked_choice_vote(candidates, votes));
    }

    #[test]
    fn ranked_choice_vote_determines_round_two_winner() {
        let candidates = vec!["Knuth", "Turing", "Church"];
        let votes = vec![
            vec![1, 0, 2],
            vec![0, 1, 2],
            vec![2, 1, 0],
            vec![2, 1, 0],
            vec![1, 2, 0],
        ];
        assert_eq!("Turing", ranked_choice_vote(candidates, votes));
    }
}
