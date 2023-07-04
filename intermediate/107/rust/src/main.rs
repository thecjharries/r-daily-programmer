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

const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz ";

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn generate_random_string<R: Rng>(length: usize, rng: &mut Rng) -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        let mut rng = Pcg64::seed_from_u64(0);
        assert_eq!("".to_string(), generate_random_string(0, &mut rng));
        assert_eq!("djtbxefnwylarfpalullcxaqglnzrcustwcotvpn sanyjljiqwfqdfinvxlpgj sqhqpelmbjsozvciegofbrinepedrnztqdrf".to_string(), generate_random_string(100, &mut rng));
        assert_eq!("veqvbewhaulynlqhibgc wwqvpbj renzowaqzkhgfqri rpsjbhpcjiovpogpswgnyinbgvx lrwduqgrkpkzg ihovfyrietrvaeqc febeyibhrizjznfiotfdflurihegvzzgxvxrzzsriwi embztmazgojv i vtxosagqdzjbofdfvfwzoehaajejc jcewat".to_string(), generate_random_string(200, &mut rng));
    }
}
