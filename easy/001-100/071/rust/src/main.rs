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

fn main() {
    println!("rad");
}

fn find_pythagorean_triplets(sum: i64) -> Vec<Vec<i64>> {
    let mut triplets = Vec::new();
    for a in 1..sum - 2 {
        for b in a..sum - a - 1 {
            let c = sum - a - b;
            if a * a + b * b == c * c {
                triplets.push(vec![a, b, c]);
            }
        }
    }
    triplets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            find_pythagorean_triplets(240),
            vec![
                vec![15, 112, 113],
                vec![40, 96, 104],
                vec![48, 90, 102],
                vec![60, 80, 100]
            ]
        );
        assert_eq!(
            find_pythagorean_triplets(504),
            vec![
                vec![15, 112, 113],
                vec![40, 96, 104],
                vec![48, 90, 102],
                vec![60, 80, 100]
            ]
        );
    }
}
