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

fn shelve_books(shelves: Vec<u32>, books: Vec<(u32, &str)>) -> Result<u32, String> {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shelve_books() {
        assert_eq!(
            Ok(2),
            shelve_books(
                vec![150, 150, 300, 150, 150],
                vec![(70, "a"), (76, "b"), (99, "c"), (75, "d"), (105, "e")]
            )
        );
        assert_eq!(
            Err("Not enough space".to_string()),
            shelve_books(
                vec![500, 500, 500],
                vec![(1309, "a"), (303, "b"), (399, "c")]
            )
        )
    }
}
