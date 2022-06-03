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
use rand::Rng;
use rand_pcg::Pcg64;

enum CardSuit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

enum CardValue {
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
    Ace,
}

struct Card {
    suit: CardSuit,
    value: CardValue,
}

struct Deck<R: Rng> {
    cards: Vec<Card>,
    rng: &mut R,
}

impl Deck {
    fn new<R: Rng>(deck_count: u8, rng: &mut R) -> Deck {
        let mut cards = Vec::new();
        for _ in 0..deck_count {
            for suit in &[
                CardSuit::Clubs,
                CardSuit::Diamonds,
                CardSuit::Hearts,
                CardSuit::Spades,
            ] {
                for value in &[
                    CardValue::Two,
                    CardValue::Three,
                    CardValue::Four,
                    CardValue::Five,
                    CardValue::Six,
                    CardValue::Seven,
                    CardValue::Eight,
                    CardValue::Nine,
                    CardValue::Ten,
                    CardValue::Jack,
                    CardValue::Queen,
                    CardValue::King,
                    CardValue::Ace,
                ] {
                    cards.push(Card {
                        suit: *suit,
                        value: *value,
                    });
                }
            }
        }
        Deck { cards, rng }
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_new() {
        let mut rng = Pcg64::from_entropy();
        let deck = Deck::new(1, &mut rng);
        assert_eq!(deck.cards.len(), 52);
    }
}
