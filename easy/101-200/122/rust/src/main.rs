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

fn compute_digital_root(number: u64) -> u64 {
    let mut current = number;
    while current > 9 {
        let mut sum = 0;
        while current > 0 {
            sum += current % 10;
            current /= 10;
        }
        current = sum;
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_digital_root() {
        assert_eq!(compute_digital_root(31337), 8);
        assert_eq!(compute_digital_root(17), 8);
        assert_eq!(compute_digital_root(0), 0);
        assert_eq!(compute_digital_root(1073741824), 1);
    }
}
