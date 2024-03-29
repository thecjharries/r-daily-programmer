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

fn funnel(first: &str, second: &str) -> bool {
    if first.len() != second.len() + 1 {
        return false;
    }
    for index in 0..first.len() {
        if first[..index].to_string() + &first[index + 1..] == second {
            return true;
        }
    }
    false
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(true, funnel("leave", "eave"));
        assert_eq!(true, funnel("reset", "rest"));
        assert_eq!(true, funnel("dragoon", "dragon"));
        assert_eq!(false, funnel("eave", "leave"));
        assert_eq!(false, funnel("sleet", "lets"));
        assert_eq!(false, funnel("skiff", "ski"));
    }
}
