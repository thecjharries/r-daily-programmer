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

use regex::Regex;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_line(line: &str, haystack: &str) -> String {
    let line_pattern =
        Regex::new(format!("(?i)(?:    .*\n)+.*{}.*(?:\n    .*)+", line.to_lowercase()).as_str())
            .unwrap();
    let line_match = line_pattern.captures(haystack).unwrap();
    line_match.get(0).unwrap().as_str().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_line() {
        assert_eq!(
            "    Fillet of a fenny snake,\n    In the caldron boil and bake;\n    Eye of newt, and toe of frog,\n    Wool of bat, and tongue of dog,\n    Adder's fork, and blind-worm's sting,\n    Lizard's leg, and howlet's wing,\n    For a charm of powerful trouble,\n    Like a hell-broth boil and bubble.",
            find_line("eye of newt", "\n  SECOND WITCH.\n    Fillet of a fenny snake,\n    In the caldron boil and bake;\n    Eye of newt, and toe of frog,\n    Wool of bat, and tongue of dog,\n    Adder's fork, and blind-worm's sting,\n    Lizard's leg, and howlet's wing,\n    For a charm of powerful trouble,\n    Like a hell-broth boil and bubble.\n\n  ALL.\n    Double, double, toil and trouble;\n    Fire, burn; and caldron, bubble.")
        );
    }
}
