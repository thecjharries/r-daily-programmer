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

fn main() {
    println!("rad");
}

fn run_bytelandian_exchange(starting_value: u64) -> u64 {
    let mut total_coins: u64 = 0;
    let mut current_coins = vec![starting_value];
    while current_coins.len() > 0 {
        let current_coin = current_coins.pop().unwrap();
        if 0 < current_coin {
            current_coins.push(current_coin / 2);
            current_coins.push(current_coin / 3);
            current_coins.push(current_coin / 4);
        } else {
            total_coins += 1;
        }
    }
    total_coins
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_bytelandian_exchange() {
        assert_eq!(run_bytelandian_exchange(7), 15);
        assert_eq!(run_bytelandian_exchange(1000), 3263);
    }
}
