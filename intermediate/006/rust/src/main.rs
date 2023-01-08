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

fn remove_repeated_substrings(input: &str) -> String {
    let mut output = String::new();
    let mut remaining = input.to_string();
    while 7 < remaining.len() {
        let mut split = remaining.len() / 2;
        let mut found = false;
        while 3 < split {
            let first = &remaining[..split];
            let second = &remaining[split..];
            if second.contains(first) {
                output.push_str(first);
                remaining = second.to_string().replace(first, "");
                found = true;
                break;
            }
            split -= 1;
        }
        if !found {
            output.push_str(&remaining[..split]);
            remaining = remaining[split..].to_string();
        }
    }
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            "aaatestBlaBla".to_string(),
            remove_repeated_substrings("aaatestBlaBlatestBlaBla")
        );
        assert_eq!(
            "aaathisBlaBla".to_string(),
            remove_repeated_substrings("aaathisBlaBlathisBlaBla")
        );
        assert_eq!(
            "aaathatBlaBla".to_string(),
            remove_repeated_substrings("aaathatBlaBlathatBlaBla")
        );
        assert_eq!(
            "aaagoodBlaBla".to_string(),
            remove_repeated_substrings("aaagoodBlaBlagoodBlaBla")
        );
        assert_eq!(
            "aaagood1BlaBla123".to_string(),
            remove_repeated_substrings("aaagood1BlaBla123good1BlaBla123")
        );
    }
}
