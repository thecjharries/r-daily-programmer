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
use std::collections::HashMap;

fn main() {
    println!("rad");
}

fn process_room_log(log: Vec<(i16, i8, char, i32)>) -> Vec<String> {
    let mut rooms: HashMap<i8, (i16, i32)> = HashMap::new();
    for (_, room_id, entry, timestamp) in log {
        if !rooms.contains_key(&room_id) {
            rooms.insert(room_id, (0, 0));
        }
        if 'I' == entry {
            (rooms.get_mut(&room_id).unwrap()).0 += 1;
            (rooms.get_mut(&room_id).unwrap()).1 -= timestamp;
        } else {
            (rooms.get_mut(&room_id).unwrap()).1 += timestamp;
        }
    }
    let mut result = Vec::new();
    for key in rooms.keys().sorted() {
        let (count, total) = rooms.get(key).unwrap();
        result.push(format!(
            "Room {}, {} minute average visit, {} visitor(s) total",
            key,
            total / *count as i32,
            count
        ));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_room_log() {
        let mut log = vec![
            (0, 0, 'I', 540),
            (1, 0, 'I', 540),
            (0, 0, 'O', 560),
            (1, 0, 'O', 560),
        ];
        assert_eq!(
            process_room_log(log),
            vec!["Room 0, 20 minute average visit, 2 visitor(s) total",]
        );
        log = vec![
            (0, 11, 'I', 347),
            (1, 13, 'I', 307),
            (2, 15, 'I', 334),
            (3, 6, 'I', 334),
            (4, 9, 'I', 334),
            (5, 2, 'I', 334),
            (6, 2, 'I', 334),
            (7, 11, 'I', 334),
            (8, 1, 'I', 334),
            (0, 11, 'O', 376),
            (1, 13, 'O', 321),
            (2, 15, 'O', 389),
            (3, 6, 'O', 412),
            (4, 9, 'O', 418),
            (5, 2, 'O', 414),
            (6, 2, 'O', 349),
            (7, 11, 'O', 418),
            (8, 1, 'O', 418),
            (0, 12, 'I', 437),
            (1, 28, 'I', 343),
            (2, 32, 'I', 408),
            (3, 15, 'I', 458),
            (4, 18, 'I', 424),
            (5, 26, 'I', 442),
            (6, 7, 'I', 435),
            (7, 19, 'I', 456),
            (8, 19, 'I', 450),
            (0, 12, 'O', 455),
            (1, 28, 'O', 374),
            (2, 32, 'O', 495),
            (3, 15, 'O', 462),
            (4, 18, 'O', 500),
            (5, 26, 'O', 479),
            (6, 7, 'O', 493),
            (7, 19, 'O', 471),
            (8, 19, 'O', 458),
        ];
        assert_eq!(
            process_room_log(log),
            vec![
                "Room 1, 84 minute average visit, 1 visitor(s) total",
                "Room 2, 47 minute average visit, 2 visitor(s) total",
                "Room 6, 78 minute average visit, 1 visitor(s) total",
                "Room 7, 58 minute average visit, 1 visitor(s) total",
                "Room 9, 84 minute average visit, 1 visitor(s) total",
                "Room 11, 56 minute average visit, 2 visitor(s) total",
                "Room 12, 18 minute average visit, 1 visitor(s) total",
                "Room 13, 14 minute average visit, 1 visitor(s) total",
                "Room 15, 29 minute average visit, 2 visitor(s) total",
                "Room 18, 76 minute average visit, 1 visitor(s) total",
                "Room 19, 11 minute average visit, 2 visitor(s) total",
                "Room 26, 37 minute average visit, 1 visitor(s) total",
                "Room 28, 31 minute average visit, 1 visitor(s) total",
                "Room 32, 87 minute average visit, 1 visitor(s) total",
            ],
        );
    }
}
