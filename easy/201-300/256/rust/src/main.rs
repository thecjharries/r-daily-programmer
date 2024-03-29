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

fn oblique(input: Vec<u32>) -> String {
    let max_size = (input.len() as f32).sqrt() as usize;
    let mut result = String::new();
    for rank in 0..max_size {
        let mut current_row = Vec::new();
        for index in 0..=rank {
            current_row.push(input[rank - index + index * max_size].to_string());
        }
        result.push_str(&format!("{}\n", current_row.join(" ")));
    }
    let mut offset = 0;
    for rank in (1..=(max_size - 1)).rev() {
        let mut current_row = Vec::new();
        for index in max_size - rank..max_size {
            current_row.push(input[max_size - index + offset + index * max_size].to_string());
        }
        offset += 1;
        result.push_str(&format!("{}\n", current_row.join(" ")));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oblique() {
        assert_eq!(
            "0\n1 6\n2 7 12\n3 8 13 18\n4 9 14 19 24\n5 10 15 20 25 30\n11 16 21 26 31\n17 22 27 32\n23 28 33\n29 34\n35\n".to_string(),
            oblique(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35])
        );
    }
}
