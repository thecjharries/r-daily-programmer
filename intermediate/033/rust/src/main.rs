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
        assert_eq!(
            Card {
                suit: Suit::Diamonds,
                value: Value::Two
            },
            Card::from_str("2D").unwrap()
        );
        assert_eq!(
            Card {
                suit: Suit::Hearts,
                value: Value::Three
            },
            Card::from_str("3H").unwrap()
        );
        assert_eq!(
            Card {
                suit: Suit::Spades,
                value: Value::Four
            },
            Card::from_str("4S").unwrap()
        );
        assert_eq!(
            Card {
                suit: Suit::Clubs,
                value: Value::Five
            },
            Card::from_str("5C").unwrap()
        );
        assert_eq!(
            Card {
                suit: Suit::Diamonds,
                value: Value::Six
            },
            Card::from_str("6D").unwrap()
        );
        assert_eq!(
            Card {
                suit: Suit::Hearts,
                value: Value::Seven
            },
            Card::from_str("7H").unwrap()
        );
        assert_eq!(
            Card {
                suit: Suit::Spades,
                value: Value::Eight
            },
            Card::from_str("8S").unwrap()
        );
        assert_eq!(
            Card {
                suit: Suit::Clubs,
                value: Value::Nine
            },
            Card::from_str("9C").unwrap()
        );
        assert_eq!(
            Card {
                suit: Suit::Diamonds,
                value: Value::Ten
            },
            Card::from_str("TD").unwrap()
        );
        assert_eq!(
            Card {
                suit: Suit::Hearts,
                value: Value::Jack
            },
            Card::from_str("JH").unwrap()
        );
        assert_eq!(
            Card {
                suit: Suit::Spades,
                value: Value::Queen
            },
            Card::from_str("QS").unwrap()
        );
        assert_eq!(
            Card {
                suit: Suit::Clubs,
                value: Value::King
            },
            Card::from_str("KC").unwrap()
        );
        assert!(Card::from_str("BQ").is_err());
        assert!(Card::from_str("Q").is_err());
    }
}
