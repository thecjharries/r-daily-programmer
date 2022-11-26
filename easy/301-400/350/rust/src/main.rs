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

use itertools::Itertools;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn shelve_books(shelves: Vec<u32>, books: Vec<(u32, &str)>) -> Result<u32, String> {
    for book in books.iter() {
        let mut found_shelf = false;
        for shelf in shelves.iter() {
            if book.0 <= *shelf {
                found_shelf = true;
                break;
            }
        }
        if !found_shelf {
            return Err("Not enough space".to_string());
        }
    }
    let total_space = shelves.iter().sum::<u32>();
    let total_book_space = books.iter().map(|book| book.0).sum::<u32>();
    if total_book_space > total_space {
        return Err("Not enough space".to_string());
    }
    for combination in shelves.iter().combinations(shelves.len()) {
        let mut book_index = 0;
        let mut shelf_index = 0;
        let mut current_shelf_size = *combination[shelf_index];
        while book_index < books.len() && shelf_index < combination.len() {
            if books[book_index].0 <= current_shelf_size {
                current_shelf_size -= books[book_index].0;
                book_index += 1;
            } else {
                shelf_index += 1;
                if shelf_index < combination.len() {
                    current_shelf_size = *combination[shelf_index];
                }
            }
            if book_index == books.len() {
                return Ok(shelf_index as u32 + 1);
            }
        }
    }
    Err("Not enough space".to_string())
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shelve_books() {
        assert_eq!(
            Ok(3),
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
        );
        assert_eq!(
            Err("Not enough space".to_string()),
            shelve_books(
                vec![100, 100],
                vec![
                    (50, "a"),
                    (50, "b"),
                    (50, "c"),
                    (50, "d"),
                    (50, "e"),
                    (50, "f")
                ]
            )
        );
        assert_eq!(
            Err("Not enough space".to_string()),
            shelve_books(vec![100, 100], vec![(60, "a"), (60, "b"), (41, "c")])
        );
    }
}
