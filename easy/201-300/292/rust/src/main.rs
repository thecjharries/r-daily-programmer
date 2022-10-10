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

fn build_range(input: &str) -> String {
    let mut numbers: Vec<String> = Vec::new();
    for chunk in input.split(",") {
        if let number @ Some(_) = chunk.parse::<u32>().ok() {
            if 0 == numbers.len() {
                numbers.push(number.unwrap().to_string());
                continue;
            }
            let mut possible = numbers[numbers.len() - 1].parse::<u32>().unwrap() + 1;
            while !possible.to_string().ends_with(chunk) {
                possible += 1;
            }
            numbers.push(possible.to_string());
        } else if chunk.contains("-") {
            let range = chunk.split("-").collect::<Vec<&str>>();
            let mut current = range[0].parse::<u32>().unwrap();
            if 0 == numbers.len() {
                numbers.push(current.to_string());
                current += 1;
            } else {
                current = numbers[numbers.len() - 1].parse::<u32>().unwrap() + 1;
                while !current.to_string().ends_with(range[0]) {
                    current += 1;
                }
            }
            while !current.to_string().ends_with(range[1]) {
                numbers.push(current.to_string());
                current += 1;
            }
            numbers.push(current.to_string());
        } else {
            let sequence = chunk.split(":").collect::<Vec<&str>>();
            let increment = sequence[2].parse::<u32>().unwrap();
            let mut current = sequence[0].parse::<u32>().unwrap();
            if 0 == numbers.len() {
                numbers.push(current.to_string());
                current += increment;
            } else {
                current = numbers[numbers.len() - 1].parse::<u32>().unwrap() + 1;
                while !current.to_string().ends_with(sequence[0]) {
                    current += increment;
                }
            }
            while !current.to_string().ends_with(sequence[1]) {
                numbers.push(current.to_string());
                current += increment;
            }
            numbers.push(current.to_string());
        }
    }
    numbers.join(" ")
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!("1 3 7 12 14 21", build_range("1,3,7,2,4,1"));
        assert_eq!("1 2 3 11 12", build_range("1-3,1-2"));
        assert_eq!("1 3 5 10 11 12 13 14", build_range("1:5:2,10:4:1"));
        assert_eq!("104 105 106 107 108 109 110 111 112", build_range("104-2"));
    }
}
