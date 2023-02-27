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

use rand::prelude::*;
use rand::Rng;
use rand_pcg::Pcg64;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn play_game<R: Rng>(rng: &mut R) -> bool {
    let mut indices: Vec<usize> = (0..8).collect::<Vec<usize>>();
    let mut results: Vec<u8> = vec![0; 8];
    for _ in 0..8 {
        let digit = rng.gen_range(0..10);
        let index = indices[((digit as f32 * indices.len() as f32) / 10.0) as usize];
        results[index] = digit;
        indices.remove(indices.iter().position(|&x| x == index).unwrap());
    }
    let mut sorted = results.clone();
    sorted.sort();
    results == sorted
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_game() {
        let mut rng = Pcg64::seed_from_u64(0);
        assert_eq!(false, play_game(&mut rng));
        rng = Pcg64::seed_from_u64(2);
        assert_eq!(true, play_game(&mut rng));
    }
}
