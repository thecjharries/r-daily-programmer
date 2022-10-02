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

fn part_two(input: Vec<u32>) -> Vec<Vec<u8>> {
    let mut output = Vec::new();
    for number in input {
        let mut number_output = Vec::new();
        if 0 == number {
            number_output.push(0);
            continue;
        }
        let mut current_number = number;
        while current_number > 0 {
            if 255 < current_number {
                number_output.push(255);
                current_number -= 255;
            } else if 255 == current_number {
                number_output.push(255);
                number_output.push(0);
                current_number = 0;
            } else {
                number_output.push(current_number as u8);
                current_number -= current_number;
            }
        }
        output.push(number_output);
    }
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        assert_eq!(vec![vec![12]], part_two(vec![12]));
        assert_eq!(vec![vec![255, 0]], part_two(vec![255]));
        assert_eq!(vec![vec![255, 1]], part_two(vec![256]));
        assert_eq!(vec![vec![255, 255, 0]], part_two(vec![510]));
        assert_eq!(
            vec![vec![255, 255, 2], vec![44], vec![255, 255, 255, 255, 4]],
            part_two(vec![512, 44, 1024])
        );
    }
}
