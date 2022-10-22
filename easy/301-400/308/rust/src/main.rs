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

fn spread_fire(map: &mut Vec<Vec<char>>, smoke: Vec<(usize, usize)>) {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spread_fire() {
        let mut map: Vec<Vec<char>> = vec![
            "#############/#".chars().collect(),
            "#     |       #".chars().collect(),
            "#     #       #".chars().collect(),
            "#     #       #".chars().collect(),
            "#######       #".chars().collect(),
            "#     _       #".chars().collect(),
            "###############".chars().collect(),
        ];
        let smoke = vec![
            (1, 1),
            (1, 2),
            (1, 3),
            (5, 6),
            (6, 2),
            (1, 1),
            (1, 2),
            (5, 5),
            (5, 5),
            (9, 1),
            (5, 7),
            (2, 2),
        ];
        spread_fire(&mut map, smoke);
        let output: Vec<Vec<char>> = vec![
            "#############/#".chars().collect(),
            "#F    |  S    #".chars().collect(),
            "#FF   #       #".chars().collect(),
            "#F    #       #".chars().collect(),
            "#######       #".chars().collect(),
            "#    F_       #".chars().collect(),
            "###############".chars().collect(),
        ];
        assert_eq!(output, map);
    }
}
