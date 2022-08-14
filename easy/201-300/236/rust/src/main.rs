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
use rand::Rng;
use rand_pcg::Pcg64;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(not(tarpaulin_include))]
fn get_pieces<R: Rng>(count: u32, rng: &mut R) -> String {
    let pool = "OISZLJT";
    let mut pieces = String::new();
    for _ in 0..count {
        pieces.push(pool.chars().nth(rng.gen_range(0..pool.len())).unwrap());
    }
    pieces
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pieces() {
        assert_eq!(
            "SOIIZJTZOLIOJOTZOILIZZTLOJLJJOZJILZSTLOZTSZSLSLILI",
            get_pieces(50, &mut Pcg64::seed_from_u64(0))
        );
    }
}
