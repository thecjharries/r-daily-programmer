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

static COINS: [u32; 6] = [1, 5, 10, 25, 100, 500];

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn change(amount: u32) -> u32 {
    let mut remaining_amount = amount;
    let mut coin_index = COINS.len() - 1;
    let mut coin_count = 0;
    while remaining_amount > 0 {
        if remaining_amount >= COINS[coin_index] {
            coin_count += remaining_amount / COINS[coin_index];
            remaining_amount %= COINS[coin_index];
        } else {
            coin_index -= 1;
        }
    }
    coin_count
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(0, change(0));
        assert_eq!(3, change(12));
        assert_eq!(11, change(468));
        assert_eq!(254, change(123456));
    }
}
