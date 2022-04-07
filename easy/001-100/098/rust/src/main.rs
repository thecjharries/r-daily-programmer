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

fn build_op_table(op: &str, max: i32) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_op_table() {
        assert_eq!(
            build_op_table("+", 2),
            "+ | 0 1 2\n---------\n0 | 0 1 2\n 1 | 1 2 3\n 2 | 2 3 4\n".to_string()
        );
        assert_eq!(
            build_op_table("*", 2),
            "* | 0 1 2\n---------\n0 | 0 0 0 \n 1 | 0 1 2\n 2 | 0 2 4\n".to_string()
        );
    }
}
