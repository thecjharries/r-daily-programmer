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

use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, PartialEq)]
enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, PartialEq)]
struct Card {
    suit: Suit,
    value: Value,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut chars = input.trim().chars();
        if 2 != input.len() {
            return Err(());
        }
        let value = match chars.next() {
            Some('A') => Value::Ace,
            Some('2') => Value::Two,
            Some('3') => Value::Three,
            Some('4') => Value::Four,
            Some('5') => Value::Five,
            Some('6') => Value::Six,
            Some('7') => Value::Seven,
            Some('8') => Value::Eight,
            Some('9') => Value::Nine,
            Some('T') => Value::Ten,
            Some('J') => Value::Jack,
            Some('Q') => Value::Queen,
            Some('K') => Value::King,
            _ => return Err(()),
        };
        let suit = match chars.next() {
            Some('C') => Suit::Clubs,
            Some('D') => Suit::Diamonds,
            Some('H') => Suit::Hearts,
            Some('S') => Suit::Spades,
            _ => return Err(()),
        };
        Ok(Card { suit, value })
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_fromstr() {
        assert_eq!(
            Card {
                suit: Suit::Clubs,
                value: Value::Ace
            },
            Card::from_str("AC").unwrap()
        );
    }
}
