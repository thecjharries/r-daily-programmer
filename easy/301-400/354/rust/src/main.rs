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

fn find_smallest_divisor_sum(input: u32) -> u32 {
    let mut output = 1 + input;
    for divisor in 2..((input as f32).sqrt() as u32 + 1) {
        if 0 == input % divisor {
            let possible_sum = divisor + input / divisor;
            if possible_sum < output {
                output = possible_sum;
            }
        }
    }
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_smallest_divisor_sum() {
        assert_eq!(7, find_smallest_divisor_sum(12));
        assert_eq!(43, find_smallest_divisor_sum(456));
        assert_eq!(4568, find_smallest_divisor_sum(4567));
        assert_eq!(838, find_smallest_divisor_sum(12345));
    }
}
