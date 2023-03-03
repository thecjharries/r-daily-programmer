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

use std::path::Path;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn tree_from_path(path: &str) -> Vec<String> {
    let mut tree = Vec::new();
    let current_path = Path::new(path);
    let components = current_path.components().collect::<Vec<_>>();
    for (index, component) in components.iter().enumerate() {
        tree.push(format!(
            "{}{}",
            "  ".repeat(index),
            component.as_os_str().to_str().unwrap()
        ));
    }
    tree
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_from_path() {
        assert_eq!(
            vec![
                "/",
                "  home",
                "    user",
                "      bin",
                "        rust",
                "          src",
                "            main.rs",
            ],
            tree_from_path("/home/user/bin/rust/src/main.rs")
        );
    }
}
