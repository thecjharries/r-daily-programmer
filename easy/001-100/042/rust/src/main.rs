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

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::Write;

const SONG_BODY: String = "Old MACDONALD had a farm
E-I-E-I-O
And on his farm he had a {animal}s
E-I-E-I-O
With a {sound}s {sound}s here
And a {sound}s {sound}s there
Here a {sound}s, there a {sound}s
Everywhere a {sound}s {sound}s
Old MacDonald had a farm
E-I-E-I-O

".to_string();

lazy_static! {
    static ref ANIMAL_TO_SOUND_MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("cow",      "moo");
	    map.insert("chicken",  "cluck");
	    map.insert("turkey",   "gobble");
	    map.insert("kangaroo", "g'day mate");
	    map.insert("T-Rex",    "GAAAAARGH");
	    map.insert("dog",      "bark");
        map
    };
}

fn main() {
    println!("rad");
}

fn generate_song(stdout: &mut Write) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
