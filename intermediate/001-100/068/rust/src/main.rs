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

fn find_minimum_throws(floors: u32, telephones: u32) -> u32 {
    let mut throws = 0;
    let mut current_floors = floors;
    let mut remaining_telephones = telephones;
    while current_floors > 1 && remaining_telephones > 1 {
        current_floors /= 2;
        remaining_telephones -= 1;
        throws += 1;
    }
    if 0 < remaining_telephones {
        throws += current_floors;
    }
    throws
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_minimum_throws() {
        assert_eq!(1, find_minimum_throws(1, 1));
        assert_eq!(2, find_minimum_throws(2, 1));
        assert_eq!(2, find_minimum_throws(3, 2));
        assert_eq!(4, find_minimum_throws(6, 2));
    }
}
