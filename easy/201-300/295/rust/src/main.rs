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

fn convert_first_to_second(first: &str, second: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut index = 0;
    result.push(first.to_string());
    while first.len() > index {
        let mut current = String::new();
        current.push_str(&second[..index]);
        while first.len() > index && first[index..index + 1] == second[index..index + 1] {
            current.push_str(&first[index..index + 1]);
            index += 1;
        }
        current.push_str(&second[index..index + 1]);
        current.push_str(&first[index + 1..]);
        result.push(current);
        index += 1;
    }
    result
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            vec!["floor", "bloor", "broor", "braor", "brakr", "brake"],
            convert_first_to_second("floor", "brake")
        );
        assert_eq!(
            vec!["wood", "bood", "book"],
            convert_first_to_second("wood", "book")
        );
        assert_eq!(
            vec![
                "a fall to the floor",
                "b fall to the floor",
                "brfall to the floor",
                "braall to the floor",
                "brakll to the floor",
                "brakil to the floor",
                "brakin to the floor",
                "brakingto the floor",
                "braking o the floor",
                "braking t the floor",
                "braking ththe floor",
                "braking thehe floor",
                "braking the e floor",
                "braking the d floor",
                "braking the dofloor",
                "braking the dooloor",
                "braking the dooroor",
                "braking the door or",
                "braking the door ir",
                "braking the door in",
            ],
            convert_first_to_second("a fall to the floor", "braking the door in")
        );
    }
}
