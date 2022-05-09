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

fn reverse(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn capitalize(s: &str) -> String {
    s.to_string().to_uppercase()
}

fn is_reversed(first: &str, second: &str) -> bool {
    first.chars().rev().collect::<String>() == second.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("rad"), "dar");
        assert_eq!(reverse("Rad"), "daR");
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("rad"), "RAD");
        assert_eq!(capitalize("Rad"), "RAD");
    }

    #[test]
    fn test_is_reversed() {
        assert_eq!(is_reversed("rad", "dar"), true);
        assert_eq!(is_reversed("rAd", "dar"), false);
        assert_eq!(is_reversed("Rad", "daR"), true);
    }

    #[test]
    fn test_is_capitalized() {
        assert_eq!(is_capitalized("rad", "RAD"), true);
        assert_eq!(is_capitalized("rad", "Rad"), false);
    }
}
