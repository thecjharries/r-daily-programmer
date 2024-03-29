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

fn players_from_money(money: u64) -> u64 {
    let mut additional_controller: u64 = 2;
    if 20 > money % 72 {
        additional_controller = 1;
    } else if 52 <= money % 72 {
        additional_controller = 3;
    }
    additional_controller + (money / 72) * 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_players_from_money() {
        assert_eq!(players_from_money(10), 1);
        assert_eq!(players_from_money(20), 2);
        assert_eq!(players_from_money(60), 3);
        assert_eq!(players_from_money(80), 4);
    }
}
