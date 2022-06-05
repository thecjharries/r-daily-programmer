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

use rand::prelude::SeedableRng;
use rand::Rng;
use rand_pcg::Pcg64;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn roll_and_build_stats<R: Rng>(count: u64, rng: &mut R) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_and_build_stats() {
        let mut rng = Pcg64::seed_from_u64(0);
        assert_eq!(
            "10         50.00% 10.00% 10.00% 00.00% 30.00% 00.00%",
            roll_and_build_stats(10, &mut rng)
        );
        assert_eq!(
            "100        21.00% 13.00% 16.00% 18.00% 16.00% 16.00%",
            roll_and_build_stats(100, &mut rng)
        );
        assert_eq!(
            "1000       17.00% 17.80% 14.60% 16.80% 16.10% 17.70%",
            roll_and_build_stats(1000, &mut rng)
        );
        assert_eq!(
            "10000      17.14% 17.16% 16.23% 16.16% 16.22% 17.09%",
            roll_and_build_stats(10000, &mut rng)
        );
    }
}
