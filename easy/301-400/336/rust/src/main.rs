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

fn cannibalize_numbers(query: u32, numbers: Vec<u32>) -> u32 {
    let mut sorted = numbers.clone();
    sorted.sort();
    let mut bottom = 0;
    let mut top = sorted.len() - 1;
    let mut result = 0;
    while bottom < top {
        if query <= sorted[top] {
            result += 1;
        } else {
            while query > sorted[top] && bottom < top {
                bottom += 1;
                sorted[top] += 1;
            }
            if query == sorted[top] {
                result += 1;
            }
        }
        top -= 1;
    }
    result
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(4, cannibalize_numbers(10, vec![21, 9, 5, 8, 10, 1, 3]));
        assert_eq!(2, cannibalize_numbers(15, vec![21, 9, 5, 8, 10, 1, 3]));
        assert_eq!(4, cannibalize_numbers(4, vec![3, 3, 3, 2, 2, 2, 1, 1, 1]));
        assert_eq!(2, cannibalize_numbers(5, vec![1, 2, 3, 4, 5]));
    }
}
