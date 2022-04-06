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

fn walk_directory_and_dump_txt_files(directory_path: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walk_directory_and_dump_txt_files() {
        assert_eq!(walk_directory_and_dump_txt_files(".."), "=== abc.txt (4 bytes)\nabc\n\n\n=== def.txt (4 bytes)\ndef\n\n\n=== ghi.txt (4 bytes)\nghi\n\n\n".to_string());
    }
}
