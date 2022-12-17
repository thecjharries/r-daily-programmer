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

fn havel_hakimi(input: Vec<u32>) -> bool {
    let mut numbers = input.into_iter().filter(|&x| x > 0).collect::<Vec<u32>>();
    if numbers.is_empty() {
        return true;
    }
    numbers.sort();
    numbers.reverse();
    let largest = numbers.pop().unwrap();
    if largest > numbers.len() as u32 {
        return false;
    }
    for index in 0..largest {
        numbers[index as usize] -= 1;
    }
    havel_hakimi(numbers)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_havel_hakimi() {
        assert_eq!(havel_hakimi(vec![5, 3, 0, 2, 6, 2, 0, 7, 2, 5]), false);
        assert_eq!(havel_hakimi(vec![4, 2, 0, 1, 5, 0]), false);
        assert_eq!(havel_hakimi(vec![3, 1, 2, 3, 1, 0]), true);
        assert_eq!(
            havel_hakimi(vec![
                16, 9, 9, 15, 9, 7, 9, 11, 17, 11, 4, 9, 12, 14, 14, 12, 17, 0, 3, 16
            ]),
            true
        );
        assert_eq!(
            havel_hakimi(vec![
                14, 10, 17, 13, 4, 8, 6, 7, 13, 13, 17, 18, 8, 17, 2, 14, 6, 4, 7, 12
            ]),
            true
        );
        assert_eq!(
            havel_hakimi(vec![
                15, 18, 6, 13, 12, 4, 4, 14, 1, 6, 18, 2, 6, 16, 0, 9, 10, 7, 12, 3
            ]),
            false
        );
        assert_eq!(
            havel_hakimi(vec![
                6, 0, 10, 10, 10, 5, 8, 3, 0, 14, 16, 2, 13, 1, 2, 13, 6, 15, 5, 1
            ]),
            false
        );
        assert_eq!(havel_hakimi(vec![2, 2, 0]), false);
        assert_eq!(havel_hakimi(vec![3, 2, 1]), false);
        assert_eq!(havel_hakimi(vec![1, 1]), true);
        assert_eq!(havel_hakimi(vec![1]), false);
        assert_eq!(havel_hakimi(vec![]), true);
    }
}
