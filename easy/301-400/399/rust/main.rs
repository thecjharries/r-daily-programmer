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
    assert_eq!(lettersum(""), 0);
    assert_eq!(lettersum("a"), 1);
    assert_eq!(lettersum("z"), 26);
    assert_eq!(lettersum("cab"), 6);
    assert_eq!(lettersum("excellent"), 100);
    assert_eq!(lettersum("microspectrophotometries"), 317);
}

fn lettersum(s: &str) -> u32 {
    s.chars().filter(|c| c.is_alphabetic()).map(|c| c.to_ascii_lowercase() as u32 - 'a' as u32 + 1).sum()
}
