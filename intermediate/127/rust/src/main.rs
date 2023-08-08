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

#[derive(Debug, PartialEq)]
struct CallForwarding {
    number: String,
    forward_to: String,
    start_day: u32,
    length: usize,
}

impl CallForwarding {
    fn new(number: &str, forward_to: &str, start_day: u32, length: usize) -> Self {
        Self {
            number: number.to_string(),
            forward_to: forward_to.to_string(),
            start_day,
            length,
        }
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn parse_data_on_day(day: u32, forwards: Vec<CallForwarding>) -> (usize, usize) {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_data_on_day_parses_count_of_forwards() {
        let forwards = vec![
            CallForwarding::new("0000", "0001", 1, 3),
            CallForwarding::new("0001", "4964", 2, 1),
            CallForwarding::new("4964", "0005", 2, 3),
        ];
        let (count, _) = parse_data_on_day(2, forwards);
        assert_eq!(3, count);
    }

    #[test]
    fn parse_data_on_day_parses_length_of_longest_chain() {
        let forwards = vec![
            CallForwarding::new("0000", "0001", 1, 3),
            CallForwarding::new("0001", "4964", 2, 1),
            CallForwarding::new("4964", "0005", 2, 3),
        ];
        let (_, length) = parse_data_on_day(2, forwards);
        assert_eq!(3, length);
    }
}
