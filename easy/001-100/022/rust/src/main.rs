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

fn union_vectors<'a>(first: Vec<&'a str>, second: Vec<&'a str>) -> Vec<&'a str> {
    let mut result = first;
    for item in second {
        if !result.contains(&item) {
            result.push(item);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_vectors() {
        assert_eq!(union_vectors(vec!["a", "b", "c"], vec!["c", "d", "e"]), vec!["a", "b", "c", "d", "e"]);
    }
}
