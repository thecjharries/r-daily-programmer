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

#[derive(Debug, PartialEq)]
enum Winner {
    Human,
    Computer,
    Neither,
}

#[derive(Debug, PartialEq)]
enum Moves {
    Rock,
    Paper,
    Scissors,
    Lizard,
    Spock,
}

impl Moves {
    fn get_winner_as_human(&self, other: &Self) -> Winner {
        match self {
            Self::Rock => match other {
                Self::Rock => Winner::Neither,
                Self::Scissors => Winner::Human,
                Self::Lizard => Winner::Human,
                _ => Winner::Computer,
            },
            Self::Paper => match other {
                Self::Paper => Winner::Neither,
                Self::Rock => Winner::Human,
                Self::Spock => Winner::Human,
                _ => Winner::Computer,
            },
            Self::Scissors => match other {
                Self::Scissors => Winner::Neither,
                Self::Paper => Winner::Human,
                Self::Lizard => Winner::Human,
                _ => Winner::Computer,
            },
            Self::Lizard => match other {
                Self::Lizard => Winner::Neither,
                Self::Paper => Winner::Human,
                Self::Spock => Winner::Human,
                _ => Winner::Computer,
            },
            Self::Spock => match other {
                Self::Spock => Winner::Neither,
                Self::Rock => Winner::Human,
                Self::Scissors => Winner::Human,
                _ => Winner::Computer,
            },
        }
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
    fn test_win_conditions() {
        assert_eq!(
            Winner::Human,
            Moves::Rock.get_winner_as_human(&Moves::Scissors)
        );
        assert_eq!(
            Winner::Human,
            Moves::Rock.get_winner_as_human(&Moves::Lizard)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Rock.get_winner_as_human(&Moves::Paper)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Rock.get_winner_as_human(&Moves::Spock)
        );
        assert_eq!(
            Winner::Neither,
            Moves::Rock.get_winner_as_human(&Moves::Rock)
        );
        assert_eq!(
            Winner::Human,
            Moves::Paper.get_winner_as_human(&Moves::Rock)
        );
        assert_eq!(
            Winner::Human,
            Moves::Paper.get_winner_as_human(&Moves::Spock)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Paper.get_winner_as_human(&Moves::Scissors)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Paper.get_winner_as_human(&Moves::Lizard)
        );
        assert_eq!(
            Winner::Neither,
            Moves::Paper.get_winner_as_human(&Moves::Paper)
        );
        assert_eq!(
            Winner::Human,
            Moves::Scissors.get_winner_as_human(&Moves::Paper)
        );
        assert_eq!(
            Winner::Human,
            Moves::Scissors.get_winner_as_human(&Moves::Lizard)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Scissors.get_winner_as_human(&Moves::Rock)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Scissors.get_winner_as_human(&Moves::Spock)
        );
        assert_eq!(
            Winner::Neither,
            Moves::Scissors.get_winner_as_human(&Moves::Scissors)
        );
        assert_eq!(
            Winner::Human,
            Moves::Lizard.get_winner_as_human(&Moves::Paper)
        );
        assert_eq!(
            Winner::Human,
            Moves::Lizard.get_winner_as_human(&Moves::Spock)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Lizard.get_winner_as_human(&Moves::Rock)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Lizard.get_winner_as_human(&Moves::Scissors)
        );
        assert_eq!(
            Winner::Neither,
            Moves::Lizard.get_winner_as_human(&Moves::Lizard)
        );
        assert_eq!(
            Winner::Human,
            Moves::Spock.get_winner_as_human(&Moves::Rock)
        );
        assert_eq!(
            Winner::Human,
            Moves::Spock.get_winner_as_human(&Moves::Scissors)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Spock.get_winner_as_human(&Moves::Paper)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Spock.get_winner_as_human(&Moves::Lizard)
        );
        assert_eq!(
            Winner::Neither,
            Moves::Spock.get_winner_as_human(&Moves::Spock)
        );
    }
}
