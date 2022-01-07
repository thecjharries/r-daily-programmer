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
use rand::distributions::Alphanumeric;
use rand_pcg::Pcg64;

fn main() {
    println!("Hello, world!");
}

fn random_password(rng: Pcg64,length: u32) -> String {
    let password: String = rng
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect();
    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_password() {
        let mut rng = Pcg64::seed_from_u64(0);
        assert_eq!(random_password(rng, 10), "1KDsyHVtHF");
        rng = Pcg64::seed_from_u64(0);
        assert_eq!(random_password(rng, 30), "1KDsyHVtHFHpJInAwXCUqgDpkqy8qD");
    }
}
