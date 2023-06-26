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

use bevy_reflect::Reflect;
use chrono::NaiveDate;
use serde::Deserialize;

const UNEMPLOYMENT_PATH: &str = "../unemployment.csv";

#[derive(Debug, Deserialize, Reflect)]
struct Record {
    #[serde(rename = "Date")]
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
    let mut results = Vec::new();
    let mut reader = csv::Reader::from_path(path).expect("Unable to read CSV");
    for result in reader.deserialize() {
        let record: Record = result.expect("Unable to deserialize record");
        results.push(record);
    }
    results
}

fn get_date_min_max(date: NaiveDate) -> Result<(f32, f32), String> {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_csv() {
        let records = read_csv(UNEMPLOYMENT_PATH);
        assert_eq!(391, records.len());
        let some_date = NaiveDate::from_ymd_opt(1980, 1, 1).unwrap();
        assert_eq!(some_date, records[0].date);
        assert!(records[0].alaska.is_some());
        assert!(records[0].alabama.is_some());
        assert!(records[0].arizona.is_some());
        assert!(records[0].arkansas.is_some());
        assert!(records[0].california.is_some());
        assert!(records[0].colorado.is_some());
        assert!(records[0].connecticut.is_some());
        assert!(records[0].delaware.is_some());
        assert!(records[0].florida.is_some());
        assert!(records[0].georgia.is_some());
        assert!(records[0].hawaii.is_some());
        assert!(records[0].iowa.is_some());
        assert!(records[0].idaho.is_some());
        assert!(records[0].illinois.is_some());
        assert!(records[0].indiana.is_some());
        assert!(records[0].kansas.is_some());
        assert!(records[0].kentucky.is_some());
        assert!(records[0].louisiana.is_some());
        assert!(records[0].massachusetts.is_some());
        assert!(records[0].maryland.is_some());
        assert!(records[0].maine.is_some());
        assert!(records[0].michigan.is_some());
        assert!(records[0].minnesota.is_some());
        assert!(records[0].missouri.is_some());
        assert!(records[0].mississippi.is_some());
        assert!(records[0].montana.is_some());
        assert!(records[0].north_carolina.is_some());
        assert!(records[0].north_dakota.is_some());
        assert!(records[0].nebraska.is_some());
        assert!(records[0].new_hampshire.is_some());
        assert!(records[0].new_jersey.is_some());
        assert!(records[0].new_mexico.is_some());
        assert!(records[0].nevada.is_some());
        assert!(records[0].new_york.is_some());
        assert!(records[0].ohio.is_some());
        assert!(records[0].oklahoma.is_some());
        assert!(records[0].oregon.is_some());
        assert!(records[0].pennsylvania.is_some());
        assert!(records[0].rhode_island.is_some());
        assert!(records[0].south_carolina.is_some());
        assert!(records[0].south_dakota.is_some());
        assert!(records[0].tennessee.is_some());
        assert!(records[0].texas.is_some());
        assert!(records[0].utah.is_some());
        assert!(records[0].virginia.is_some());
        assert!(records[0].vermont.is_some());
        assert!(records[0].washington.is_some());
        assert!(records[0].wisconsin.is_some());
        assert!(records[0].west_virginia.is_some());
        assert!(records[0].wyoming.is_some());
    }
}
