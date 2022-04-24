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

use rand::prelude::*;
use rand_pcg::Pcg64;
use std::io::{Error, Read, Write};

fn main() {
    println!("rad");
}

fn play_guessing_game<R: Rng>(rng: &mut R, input: &mut impl Read, output: &mut impl Write) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_guessing_game() {
        assert_eq!(2 + 2, 4);
    }
}
