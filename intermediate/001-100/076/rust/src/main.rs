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
use std::collections::HashMap;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn graph(f: &mut dyn FnMut() -> u32, count: u32) -> Vec<String> {
    let mut results: HashMap<u32, u32> = HashMap::new();
    for _ in 0..count {
        let result = f();
        *results.entry(result).or_insert(0) += 1;
    }
    let mut max = 0;
    for (_, value) in results.iter() {
        if *value > max {
            max = *value;
        }
    }
    let mut output: Vec<String> = Vec::new();
    for (key, value) in results {
        let mut line = String::new();
        line.push_str(&format!("{:5} = ", key));
        for _ in 0..(value * 20 / max) {
            line.push_str("#");
        }
        output.push(line);
    }
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        let mut rng = Pcg64::seed_from_u64(0);
        let f: &mut dyn FnMut() -> u32 = &mut || rng.gen_range(1..=6) + rng.gen_range(1..=6);
        let count = 10000;
        let result = graph(f, count);
        assert_eq!(11, result.len());
    }
}
