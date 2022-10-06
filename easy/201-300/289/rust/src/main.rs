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

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct PokemonType {
    name: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct PokemonTypeDamageRelations {
    double_damage_from: Vec<PokemonType>,
    double_damage_to: Vec<PokemonType>,
    half_damage_from: Vec<PokemonType>,
    half_damage_to: Vec<PokemonType>,
    no_damage_from: Vec<PokemonType>,
    no_damage_to: Vec<PokemonType >,
}

#[derive(Deserialize, Debug)]
struct PokemonTypeQuery {
    damage_relations: PokemonTypeDamageRelations,
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

async fn determine_damage_multipler(attack_type: &str, defender_types: Vec<&str>) -> Result<f32, Error> {
    let mut damage_multiplier = 1.0;
    for defender_type in defender_types {
        let request_url = format!("https://pokeapi.co/api/v2/type/{}", defender_type);
        let response = reqwest::get(&request_url).await?;
        let users: PokemonTypeQuery = response.json().await?;
        for no_damage_from in &users.damage_relations.no_damage_from {
            if attack_type == no_damage_from.name {
                return Ok(0.0);
            }
        }
        for double_damage_from in &users.damage_relations.double_damage_from {
            if attack_type == double_damage_from.name {
                damage_multiplier *= 2.0;
            }
        }
        for half_damage_from in &users.damage_relations.half_damage_from {
            if attack_type == half_damage_from.name {
                damage_multiplier /= 2.0;
            }
        }
    }
    Ok(damage_multiplier)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stub() {
        assert_eq!(2.0, determine_damage_multipler("fire", vec!["grass"]).await.unwrap());
        assert_eq!(4.0, determine_damage_multipler("fighting", vec!["ice", "rock"]).await.unwrap());
        assert_eq!(0.0, determine_damage_multipler("psychic", vec!["poison", "dark"]).await.unwrap());
        assert_eq!(1.0, determine_damage_multipler("water", vec!["normal"]).await.unwrap());
        assert_eq!(0.5, determine_damage_multipler("fire", vec!["rock"]).await.unwrap());
    }
}
