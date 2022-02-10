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

fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
