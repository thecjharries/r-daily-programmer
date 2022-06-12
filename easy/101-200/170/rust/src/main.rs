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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn determine_hand_score(hand: Vec<Card>) -> u8 {
    let mut ace_count = 0;
    let mut score = 0;
    for card in hand.iter() {
        match card.value {
            CardValue::Two => score += 2,
            CardValue::Three => score += 3,
            CardValue::Four => score += 4,
            CardValue::Five => score += 5,
            CardValue::Six => score += 6,
            CardValue::Seven => score += 7,
            CardValue::Eight => score += 8,
            CardValue::Nine => score += 9,
            CardValue::Ten => score += 10,
            CardValue::Jack => score += 10,
            CardValue::Queen => score += 10,
            CardValue::King => score += 10,
            CardValue::Ace => {
                ace_count += 1;
            }
        }
    }
    for _ in 0..ace_count {
        if score + 11 > 21 {
            score += 1;
        } else {
            score += 11;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
