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

fn morse(size: usize) -> Vec<String> {
    if size <= 0 {
        return Vec::new();
    }
    if size == 1 {
        return vec![".".to_string()];
    }
    if size == 2 {
        return vec!["..".to_string(), "-".to_string()];
    }
    let dots = morse(size - 1);
    let mut result = Vec::new();
    for dot in dots {
        result.push(dot + ".");
    }
    let dashes = morse(size - 2);
    for dash in dashes {
        result.push(dash + "-");
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(morse(0), Vec::new());
        assert_eq!(morse(1), vec![".".to_string()]);
        assert_eq!(morse(2), vec!["..".to_string(), "-".to_string()]);
        assert_eq!(
            morse(3),
            vec!["...".to_string(), ".-".to_string(), "-.".to_string()]
        );
        assert_eq!(
            morse(4),
            vec![
                "....".to_string(),
                "..-".to_string(),
                ".-.".to_string(),
                "-..".to_string(),
                "--".to_string()
            ]
        );
        assert_eq!(
            morse(5),
            vec![
                ".....".to_string(),
                "...-".to_string(),
                "..-.".to_string(),
                ".-..".to_string(),
                "-...".to_string(),
                ".--".to_string(),
                "-.-".to_string(),
                "--.".to_string()
            ]
        );
    }
}
