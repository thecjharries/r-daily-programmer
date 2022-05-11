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

fn process_room_log(log: Vec<(i8, i16, char, i32)>) -> Vec<String> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_room_log() {
        let log = vec![
            (0, 0, 'I', 540),
            (1, 0, 'I', 540),
            (0, 0, 'O', 560),
            (1, 0, 'O', 560),
        ];
        assert_eq!(
            process_room_log(log),
            vec!["Room 0, 20 minute average visit, 2 visitor(s) total"]
        );
    }
}
