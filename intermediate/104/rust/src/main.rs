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

fn brackets_are_closed(input: &str) -> bool {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brackets_are_closed() {
        assert!(brackets_are_closed("()"));
        assert!(brackets_are_closed("123"));
        assert!(brackets_are_closed("((3^2 + 8)*(5/2))/(2+6)"));
        assert!(!brackets_are_closed("(abc[123)abc]"));
        assert!(!brackets_are_closed("(abc)abc]"));
    }
}
