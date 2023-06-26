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

use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    date: NaiveDate,
    #[serde(rename = "AK")]
    alaska: Option<f32>,
    #[serde(rename = "AL")]
    alabama: Option<f32>,
    #[serde(rename = "AR")]
    arizona: Option<f32>,
    #[serde(rename = "AZ")]
    arkansas: Option<f32>,
    #[serde(rename = "CA")]
    california: Option<f32>,
    #[serde(rename = "CO")]
    colorado: Option<f32>,
    #[serde(rename = "CT")]
    connecticut: Option<f32>,
    #[serde(rename = "DE")]
    delaware: Option<f32>,
    #[serde(rename = "FL")]
    florida: Option<f32>,
    #[serde(rename = "GA")]
    georgia: Option<f32>,
    #[serde(rename = "HI")]
    hawaii: Option<f32>,
    #[serde(rename = "IA")]
    iowa: Option<f32>,
    #[serde(rename = "ID")]
    idaho: Option<f32>,
    #[serde(rename = "IL")]
    illinois: Option<f32>,
    #[serde(rename = "IN")]
    indiana: Option<f32>,
    #[serde(rename = "KS")]
    kansas: Option<f32>,
    #[serde(rename = "KY")]
    kentucky: Option<f32>,
    #[serde(rename = "LA")]
    louisiana: Option<f32>,
    #[serde(rename = "MA")]
    massachusetts: Option<f32>,
    #[serde(rename = "MD")]
    maryland: Option<f32>,
    #[serde(rename = "ME")]
    maine: Option<f32>,
    #[serde(rename = "MI")]
    michigan: Option<f32>,
    #[serde(rename = "MN")]
    minnesota: Option<f32>,
    #[serde(rename = "MO")]
    missouri: Option<f32>,
    #[serde(rename = "MS")]
    mississippi: Option<f32>,
    #[serde(rename = "MT")]
    montana: Option<f32>,
    #[serde(rename = "NC")]
    north_carolina: Option<f32>,
    #[serde(rename = "ND")]
    north_dakota: Option<f32>,
    #[serde(rename = "NE")]
    nebraska: Option<f32>,
    #[serde(rename = "NH")]
    new_hampshire: Option<f32>,
    #[serde(rename = "NJ")]
    new_jersey: Option<f32>,
    #[serde(rename = "NM")]
    new_mexico: Option<f32>,
    #[serde(rename = "NV")]
    nevada: Option<f32>,
    #[serde(rename = "NY")]
    new_york: Option<f32>,
    #[serde(rename = "OH")]
    ohio: Option<f32>,
    #[serde(rename = "OK")]
    oklahoma: Option<f32>,
    #[serde(rename = "OR")]
    oregon: Option<f32>,
    #[serde(rename = "PA")]
    pennsylvania: Option<f32>,
    #[serde(rename = "RI")]
    rhode_island: Option<f32>,
    #[serde(rename = "SC")]
    south_carolina: Option<f32>,
    #[serde(rename = "SD")]
    south_dakota: Option<f32>,
    #[serde(rename = "TN")]
    tennessee: Option<f32>,
    #[serde(rename = "TX")]
    texas: Option<f32>,
    #[serde(rename = "UT")]
    utah: Option<f32>,
    #[serde(rename = "VA")]
    virginia: Option<f32>,
    #[serde(rename = "VT")]
    vermont: Option<f32>,
    #[serde(rename = "WA")]
    washington: Option<f32>,
    #[serde(rename = "WI")]
    wisconsin: Option<f32>,
    #[serde(rename = "WV")]
    west_virginia: Option<f32>,
    #[serde(rename = "WY")]
    wyoming: Option<f32>,
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn read_csv(path: &str) -> Vec<Record> {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
