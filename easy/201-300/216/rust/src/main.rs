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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CardSuit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Card {
    suit: CardSuit,
    value: CardValue,
}

struct Deck<'a, R: Rng> {
    cards: Vec<Card>,
    rng: &'a mut R,
}

impl<'a, R: Rng> Deck<'a, R> {
    fn new(deck_count: u8, rng: &'a mut R) -> Deck<'a, R> {
        let mut cards = Vec::new();
        for _ in 0..deck_count {
            // Tarpaulin doesn't register coverage on the arrays
            #[cfg(not(tarpaulin_include))]
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

    fn shuffle(&mut self) {
        for index in 0..self.cards.len() - 1 {
            let swap_index = self.rng.gen_range(index..self.cards.len());
            self.cards.swap(index, swap_index);
        }
    }

    // Tarpaulin doesn't register coverage on the return value
    #[cfg(not(tarpaulin_include))]
    fn deal(&mut self, count: u8) -> Vec<Card> {
        let mut dealt = Vec::new();
        for _ in 0..count {
            dealt.push(self.cards.pop().unwrap());
        }
        dealt
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn deal_poker_hands<R: Rng>(deck: &mut Deck<R>, hands: u8) -> Vec<Vec<Card>> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_new() {
        let mut rng = Pcg64::from_entropy();
        let deck = Deck::new(1, &mut rng);
        assert_eq!(52, deck.cards.len());
        assert_eq!(
            Card {
                suit: CardSuit::Clubs,
                value: CardValue::Two
            },
            deck.cards[0]
        );
    }

    #[test]
    fn test_deck_shuffle() {
        let mut rng = StdRng::seed_from_u64(0);
        let mut deck = Deck::new(1, &mut rng);
        assert_eq!(deck.cards.len(), 52);
        assert_eq!(
            Card {
                suit: CardSuit::Clubs,
                value: CardValue::Two
            },
            deck.cards[0]
        );
        deck.shuffle();
        assert_eq!(
            Card {
                suit: CardSuit::Hearts,
                value: CardValue::Ace
            },
            deck.cards[0]
        );
    }

    #[test]
    fn test_deck_deal() {
        let mut rng = StdRng::seed_from_u64(0);
        let mut deck = Deck::new(1, &mut rng);
        assert_eq!(52, deck.cards.len());
        assert_eq!(
            deck.cards[51],
            Card {
                suit: CardSuit::Spades,
                value: CardValue::Ace
            }
        );
        let dealt = deck.deal(5);
        assert_eq!(5, dealt.len());
        assert_eq!(47, deck.cards.len());
        assert_eq!(
            Card {
                suit: CardSuit::Spades,
                value: CardValue::Nine
            },
            deck.cards[46]
        );
        assert_eq!(
            Card {
                suit: CardSuit::Spades,
                value: CardValue::Ace
            },
            dealt[0]
        );
        let empty = deck.deal(0);
        assert_eq!(0, empty.len());
    }

    #[test]
    fn test_deal_poker_hands() {
        let mut rng = StdRng::seed_from_u64(0);
        let mut deck = Deck::new(1, &mut rng);
        assert_eq!(52, deck.cards.len());
        assert_eq!(
            deck.cards[51],
            Card {
                suit: CardSuit::Spades,
                value: CardValue::Ace
            }
        );
        let hands = deal_poker_hands(&mut deck, 5);
        assert_eq!(5, hands.len());
        assert_eq!(52 - 25, deck.cards.len());
        assert_eq!(
            Card {
                suit: CardSuit::Hearts,
                value: CardValue::Two
            },
            deck.cards[deck.cards.len() - 1]
        );
        assert_eq!(
            Card {
                suit: CardSuit::Spades,
                value: CardValue::Ace
            },
            hands[0][0]
        );
        for index in 0..hands.len() {
            assert_eq!(5, hands[index].len());
        }
    }
}
