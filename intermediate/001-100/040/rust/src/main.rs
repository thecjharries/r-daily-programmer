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

fn compute_kaprekar_chain(input: u32) -> Vec<u32> {
    let mut chain = vec![input];
    loop {
        let mut digits = chain
            .last()
            .unwrap()
            .to_string()
            .chars()
            .collect::<Vec<char>>();
        digits.sort();
        let ascending = digits.iter().collect::<String>().parse::<u32>().unwrap();
        digits.reverse();
        let descending = digits.iter().collect::<String>().parse::<u32>().unwrap();
        let next = descending - ascending;
        if chain.contains(&next) {
            break;
        }
        chain.push(next);
    }
    chain
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_kaprekar_chain() {
        assert_eq!(vec![3524, 3087, 8352, 6174], compute_kaprekar_chain(3524));
    }
}
