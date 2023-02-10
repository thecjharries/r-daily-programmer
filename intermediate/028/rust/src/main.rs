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

fn find_tetrahedral_number(input: u64) -> u64 {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_tetrahedral_number() {
        assert_eq!(0, find_tetrahedral_number(0));
        assert_eq!(1, find_tetrahedral_number(1));
        assert_eq!(3, find_tetrahedral_number(2));
        assert_eq!(3, find_tetrahedral_number(3));
        assert_eq!(3, find_tetrahedral_number(4));
        assert_eq!(6, find_tetrahedral_number(5));
        assert_eq!(6, find_tetrahedral_number(6));
        assert_eq!(6, find_tetrahedral_number(7));
        assert_eq!(6, find_tetrahedral_number(8));
        assert_eq!(505013002528, find_tetrahedral_number(169179692512835000));
    }
}
