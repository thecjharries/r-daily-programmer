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

fn find_years_with_no_repeated_digits(start: u32, end: u32) -> Vec<u32> {
    let mut result = Vec::new();
    for year in start..end + 1 {
        let mut digits = format!("{}", year).chars().collect::<Vec<char>>();
        digits.sort();
        digits.dedup();
        if digits.len() == 4 {
            result.push(year);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_years_with_no_repeated_digits() {
        assert_eq!(
            find_years_with_no_repeated_digits(1980, 1987),
            vec![1980, 1982, 1983, 1984, 1985, 1986, 1987]
        );
    }
}
