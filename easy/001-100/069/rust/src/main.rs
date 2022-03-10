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

fn build_table(title: String, rows: Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_table() {
        assert_eq!(
            build_table(
                "Necessities".to_string(),
                vec![
                    "fairy".to_string(),
                    "cakes".to_string(),
                    "happy".to_string(),
                    "fish".to_string(),
                    "disgustipated".to_string(),
                    "melon-balls".to_string()
                ]
            ),
            "+---------------+
|  Necessities  |
+---------------+
| fairy         |
| cakes         |
| happy         |
| fish          |
| disgustipated |
| melon-balls   |
+---------------+"
        )
    }
}
