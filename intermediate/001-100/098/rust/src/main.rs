// Copyright 2023 CJ Harries
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

fn multiple_count(limit: u32, numbers: Vec<u32>) -> u32 {
    let mut count = 0;
    let mut index = 0;
    let mut current = 0;
    while current < limit {
        current += numbers[index] - current % numbers[index];
        index = (index + 1) % numbers.len();
        count += 1;
    }
    count
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_count() {
        assert_eq!(1, multiple_count(5, vec![5, 7, 3]));
        assert_eq!(2, multiple_count(7, vec![5, 7, 3]));
        assert_eq!(3, multiple_count(9, vec![5, 7, 3]));
        assert_eq!(4, multiple_count(10, vec![5, 7, 3]));
        assert_eq!(5, multiple_count(14, vec![5, 7, 3]));
        assert_eq!(6, multiple_count(15, vec![5, 7, 3]));
        assert_eq!(
            408041,
            multiple_count(1000000000, vec![5395, 7168, 2367, 9999, 3])
        );
    }
}
