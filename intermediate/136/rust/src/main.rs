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
    let required_votes = (votes.len() as f32 / 2.0).ceil() as usize;
    let mut candidate_votes = vec![0; candidates.len()];
    for vote in votes.iter() {
        candidate_votes[vote[0]] += 1;
        if candidate_votes[vote[0]] >= required_votes {
            return candidates[vote[0]].to_string();
        }
    }
    let lowest_candidate = candidate_votes
        .iter()
        .enumerate()
        .min_by_key(|(_, &votes)| votes)
        .unwrap()
        .0;
    let mut lowest_candidates = vec![lowest_candidate];
    while lowest_candidates.len() < candidates.len() - 1 {
        candidate_votes = vec![0; candidates.len()];
        for vote in votes.iter() {
            let mut vote_index = 0;
            while lowest_candidates.contains(&vote[vote_index]) {
                vote_index += 1;
            }
            candidate_votes[vote[vote_index]] += 1;
            if candidate_votes[vote[vote_index]] >= required_votes {
                return candidates[vote[vote_index]].to_string();
            }
        }
        let lowest_candidate = candidate_votes
            .iter()
            .enumerate()
            .min_by_key(|(_, &votes)| votes)
            .unwrap()
            .0;
        lowest_candidates.push(lowest_candidate);
    }
    unreachable!()
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
