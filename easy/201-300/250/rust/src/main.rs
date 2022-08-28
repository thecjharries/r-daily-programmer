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

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::read_to_string;

#[derive(Serialize, Deserialize)]
struct RawPost {
    title: String,
    url: String,
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn parse_raw_posts(raw_posts: &str) -> Result<Vec<RawPost>> {
    serde_json::from_str(raw_posts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_raw_posts() {
        let raw_posts_body = read_to_string("../simplified.json").unwrap();
        assert_ne!("", raw_posts_body);
        let raw_posts = parse_raw_posts(&raw_posts_body).unwrap();
        assert_eq!(25, raw_posts.len());
        assert_eq!(
            "[2021-07-12] Challenge #398 [Difficult] Matrix Sum",
            raw_posts[0].title
        );
        assert_eq!("https://www.reddit.com/r/dailyprogrammer/comments/oirb5v/20210712_challenge_398_difficult_matrix_sum/", raw_posts[0].url);
    }
}
