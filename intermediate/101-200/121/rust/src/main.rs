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

use memoize::memoize;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[memoize]
fn break_coin(input: u64) -> (u64, u64, u64) {
    (input / 2, input / 3, input / 4)
}

#[memoize]
fn get_max_coin_value(coin: u64) -> u64 {
    if 5 > coin {
        return coin;
    }
    let (half, third, fourth) = break_coin(coin);
    if half + third + fourth > coin {
        get_max_coin_value(half) + get_max_coin_value(third) + get_max_coin_value(fourth)
    } else {
        coin
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coins_should_break_down_properly() {
        assert_eq!((10, 6, 5), break_coin(20));
    }

    #[test]
    fn coin_value_should_return_appropriate_value() {
        assert_eq!(21, get_max_coin_value(20));
        assert_eq!(1370, get_max_coin_value(1000));
        assert_eq!(51544065905, get_max_coin_value(10000000000));
    }
}
