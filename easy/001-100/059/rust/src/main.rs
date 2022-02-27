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

fn find_first_index_of(needle: &str, haystack: &str) -> i32 {
    if needle.len() > haystack.len() {
        return -1;
    }
    for i in 0..haystack.len() - needle.len() + 1 {
        if &haystack[i..i + needle.len()] == needle {
            return i as i32;
        }
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_index_of() {
        assert_eq!(find_first_index_of("rad", "rad"), 0);
        assert_eq!(
            find_first_index_of("il an", "Double, double, toil and trouble"),
            18
        );
        assert_eq!(find_first_index_of("qqq", "rad"), -1);
    }
}
