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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn same_necklace(first: &str, second: &str) -> bool {
    assert_eq!(true, same_necklace("nicole", "icolen"));
    assert_eq!(true, same_necklace("nicole", "lenico"));
    assert_eq!(false, same_necklace("nicole", "coneli"));
    assert_eq!(true, same_necklace("aabaaaaabaab", "aabaabaabaaa"));
    assert_eq!(false, same_necklace("abc", "cba"));
    assert_eq!(false, same_necklace("xxyyy", "xxxyy"));
    assert_eq!(false, same_necklace("xyxxz", "xxyxz"));
    assert_eq!(true, same_necklace("x", "x"));
    assert_eq!(false, same_necklace("x", "xx"));
    assert_eq!(false, same_necklace("x", ""));
    assert_eq!(true, same_necklace("", ""));
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
