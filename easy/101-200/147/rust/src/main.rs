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

fn is_american_football_score_valid(score: i32) -> bool {
    if 0 == score || 3 == score || 5 < score {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(is_american_football_score_valid(-1), false);
        assert_eq!(is_american_football_score_valid(0), true);
        assert_eq!(is_american_football_score_valid(1), false);
        assert_eq!(is_american_football_score_valid(2), false);
        assert_eq!(is_american_football_score_valid(3), true);
        assert_eq!(is_american_football_score_valid(4), false);
        assert_eq!(is_american_football_score_valid(5), false);
        assert_eq!(is_american_football_score_valid(6), true);
        assert_eq!(is_american_football_score_valid(35), true);
    }
}
