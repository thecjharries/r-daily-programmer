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

use glob::glob;

fn main() {
    println!("rad");
}

fn walk_directory_and_dump_txt_files(directory_path: &str) -> String {
    let mut result = String::new();
    for entry in
        glob(format!("{}/*.txt", directory_path).as_str()).expect("Failed to read glob pattern")
    {
        let path = match entry {
            Ok(path) => path,
            Err(_e) => {
                continue;
            }
        };
        result.push_str(
            format!(
                "=== {} ({} bytes)\n{}\n\n",
                path.file_name().unwrap().to_str().unwrap(),
                path.metadata().unwrap().len(),
                &std::fs::read_to_string(path.to_str().unwrap()).expect("Failed to read file"),
            )
            .as_str(),
        );
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walk_directory_and_dump_txt_files() {
        assert_eq!(walk_directory_and_dump_txt_files(".."), "=== abc.txt (4 bytes)\nabc\n\n\n=== def.txt (4 bytes)\ndef\n\n\n=== ghi.txt (4 bytes)\nghi\n\n\n".to_string());
    }
}
