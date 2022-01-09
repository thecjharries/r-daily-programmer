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

fn generate_lyrics(bottles: i32) -> String {
    let mut lyrics = String::new();
    for i in (0..=bottles).rev() {
        if i == 0 {
            lyrics.push_str("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
        } else if i == 1 {
            lyrics.push_str(&format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", i, i));
        } else {
            lyrics.push_str(&format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", i, i, i-1));
        }
    }
    lyrics
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_lyrics() {
        assert_eq!(generate_lyrics(0), "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
        assert_eq!(generate_lyrics(1), "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\nNo more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
        assert_eq!(generate_lyrics(2), "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottles of beer on the wall.\n1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\nNo more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
        assert_eq!(generate_lyrics(-10), "");
    }
}
