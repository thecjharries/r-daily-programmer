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
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io;

const SONG_BODY: &'static str = "Old MACDONALD had a farm
E-I-E-I-O
And on his farm he had a {animal}s
E-I-E-I-O
With a {sound}s {sound}s here
And a {sound}s {sound}s there
Here a {sound}s, there a {sound}s
Everywhere a {sound}s {sound}s
Old MacDonald had a farm
E-I-E-I-O

";

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
    generate_song(&mut io::stdout());
}

fn generate_song(stdout: &mut dyn io::Write) {
    for animal in ANIMAL_TO_SOUND_MAP.keys().sorted() {
        let mut song = SONG_BODY.clone().to_string();
        song = song.replace("{animal}", animal);
        song = song.replace("{sound}", ANIMAL_TO_SOUND_MAP[animal]);
        write!(stdout, "{}", song).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_song() {
        let mut stdout = Vec::new();
        generate_song(&mut stdout);
        assert_eq!(String::from_utf8(stdout).expect("Invalid"), "Old MACDONALD had a farm
E-I-E-I-O
And on his farm he had a cows
E-I-E-I-O
With a moos moos here
And a moos moos there
Here a moos, there a moos
Everywhere a moos moos
Old MacDonald had a farm
E-I-E-I-O

Old MACDONALD had a farm
E-I-E-I-O
And on his farm he had a dogs
E-I-E-I-O
With a barks barks here
And a barks barks there
Here a barks, there a barks
Everywhere a barks barks
Old MacDonald had a farm
E-I-E-I-O

Old MACDONALD had a farm
E-I-E-I-O
And on his farm he had a chickens
E-I-E-I-O
With a clucks clucks here
And a clucks clucks there
Here a clucks, there a clucks
Everywhere a clucks clucks
Old MacDonald had a farm
E-I-E-I-O

Old MACDONALD had a farm
E-I-E-I-O
And on his farm he had a turkeys
E-I-E-I-O
With a gobbles gobbles here
And a gobbles gobbles there
Here a gobbles, there a gobbles
Everywhere a gobbles gobbles
Old MacDonald had a farm
E-I-E-I-O

Old MACDONALD had a farm
E-I-E-I-O
And on his farm he had a T-Rexs
E-I-E-I-O
With a GAAAAARGHs GAAAAARGHs here
And a GAAAAARGHs GAAAAARGHs there
Here a GAAAAARGHs, there a GAAAAARGHs
Everywhere a GAAAAARGHs GAAAAARGHs
Old MacDonald had a farm
E-I-E-I-O

Old MACDONALD had a farm
E-I-E-I-O
And on his farm he had a kangaroos
E-I-E-I-O
With a g'day mates g'day mates here
And a g'day mates g'day mates there
Here a g'day mates, there a g'day mates
Everywhere a g'day mates g'day mates
Old MacDonald had a farm
E-I-E-I-O

".to_string());
    }
}
