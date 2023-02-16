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

use rand::Rng;
use rand::SeedableRng;
use rand_pcg::Pcg64;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn flip<R: Rng>(rng: &mut R) -> bool {
    rng.gen_range(0..2) == 1
}

fn generate_random<R: Rng>(max: u32, rng: &mut R) -> u32 {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip() {
        let mut rng = Pcg64::seed_from_u64(0);
        assert_eq!(false, flip(&mut rng));
        assert_eq!(false, flip(&mut rng));
        assert_eq!(true, flip(&mut rng));
    }

    #[test]
    fn test_generate_random() {
        let mut rng = Pcg64::seed_from_u64(0);
        assert_eq!(0, generate_random(1, &mut rng));
        assert_eq!(0, generate_random(2, &mut rng));
        assert_eq!(0, generate_random(2, &mut rng));
        assert_eq!(1, generate_random(2, &mut rng));
        assert_eq!(17, generate_random(50, &mut rng));
    }
}
