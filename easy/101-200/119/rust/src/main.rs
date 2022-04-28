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

fn convert_to_change(amount: f32) -> Vec<u32> {
    let mut current_amount = amount;
    let mut change = Vec::new();
    let coins = vec![0.25, 0.10, 0.05, 0.01];
    for coin in coins {
        let mut coin_count = 0;
        while current_amount >= coin {
            coin_count += 1;
            current_amount -= coin;
        }
        println!("{}", current_amount);
        change.push(coin_count);
    }
    change
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_change() {
        assert_eq!(convert_to_change(4.17), vec![16, 1, 1, 2]);
    }
}
