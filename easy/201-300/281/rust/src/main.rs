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

fn find_smallest_base(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_smallest_base() {
        assert_eq!("base 2 => 1", find_smallest_base("1"));
        assert_eq!("base 3 => 7", find_smallest_base("21"));
        assert_eq!("base 12 => 1575", find_smallest_base("ab3"));
        assert_eq!("base 16 => 255", find_smallest_base("ff"));
    }
}
