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

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref LETTER_TO_NUMBER: HashMap<char, (f32, f32)> = HashMap::from_iter([
        ('1', (0.0, 0.0)),
        ('2', (1.0, 0.0)),
        ('3', (2.0, 0.0)),
        ('4', (0.0, 1.0)),
        ('5', (1.0, 1.0)),
        ('6', (2.0, 1.0)),
        ('7', (0.0, 2.0)),
        ('8', (1.0, 2.0)),
        ('9', (2.0, 2.0)),
        ('.', (0.0, 3.0)),
        ('0', (1.0, 3.0)),
        ('#', (2.0, 3.0)),
    ]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn calculate_ip_distance(ip: &str) -> f32 {
    let mut distance = 0.0;
    let chars = ip.chars().collect::<Vec<char>>();
    for index in 1..chars.len() {
        let (x1, y1) = LETTER_TO_NUMBER.get(&chars[index - 1]).unwrap();
        let (x2, y2) = LETTER_TO_NUMBER.get(&chars[index]).unwrap();
        distance += ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
    }
    (distance * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_ip_distance() {
        assert_eq!(27.38, calculate_ip_distance("219.45.143.143"));
    }
}
