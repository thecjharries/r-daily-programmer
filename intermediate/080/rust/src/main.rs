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

#[derive(Debug, PartialEq, Clone)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, PartialEq, Clone)]
enum Rank {
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
    Knight,
    Queen,
    King,
}

#[derive(Debug, PartialEq, Clone)]
struct Card {
    suit: Suit,
    rank: Rank,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let characters = s.to_uppercase().chars().collect::<Vec<char>>();
        if 3 != characters.len() && 2 != characters.len() {
            return Err("Invalid length".to_string());
        }
        let suit = match characters[characters.len() - 1] {
            'H' => Suit::Hearts,
            'S' => Suit::Spades,
            'C' => Suit::Clubs,
            'D' => Suit::Diamonds,
            _ => return Err("Unknown suit".to_string()),
        };
        let rank = match characters[0] {
            'A' => Rank::Ace,
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            '1' => {
                if '0' != characters[1] {
                    return Err("Invalid rank".to_string());
                }
                Rank::Ten
            }
            'J' => Rank::Knight,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            _ => return Err("Unknown rank".to_string()),
        };
        Ok(Card { suit, rank })
    }
}

#[derive(Debug, PartialEq, Clone)]
struct PokerHand([Card; 5]);

impl FromStr for PokerHand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .split_whitespace()
            .map(|card| Card::from_str(card))
            .collect::<Result<Vec<Card>, String>>()?;
        if 5 != cards.len() {
            return Err("Invalid number of cards".to_string());
        }
        Ok(PokerHand([
            cards[0].clone(),
            cards[1].clone(),
            cards[2].clone(),
            cards[3].clone(),
            cards[4].clone(),
        ]))
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
    fn test_card_from_str() {
        assert_eq!(
            Card {
                suit: Suit::Hearts,
                rank: Rank::Ten
            },
            Card::from_str("10H").unwrap()
        );
    }

    #[test]
    fn test_pokerhand_from_str() {
        let hand = PokerHand([
            Card {
                suit: Suit::Hearts,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::King,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Knight,
            },
        ]);
        assert_eq!(hand, PokerHand::from_str("10H AH QH KH JH").unwrap());
    }
}
