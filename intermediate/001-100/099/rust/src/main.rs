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
use std::ops::Index;

const UNEMPLOYMENT_PATH: &str = "../unemployment.csv";

#[derive(Debug, Deserialize)]
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

impl Index<usize> for Record {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => self.alaska.as_ref().unwrap_or(&0.0),
            1 => self.alabama.as_ref().unwrap_or(&0.0),
            2 => self.arizona.as_ref().unwrap_or(&0.0),
            3 => self.arkansas.as_ref().unwrap_or(&0.0),
            4 => self.california.as_ref().unwrap_or(&0.0),
            5 => self.colorado.as_ref().unwrap_or(&0.0),
            6 => self.connecticut.as_ref().unwrap_or(&0.0),
            7 => self.delaware.as_ref().unwrap_or(&0.0),
            8 => self.florida.as_ref().unwrap_or(&0.0),
            9 => self.georgia.as_ref().unwrap_or(&0.0),
            10 => self.hawaii.as_ref().unwrap_or(&0.0),
            11 => self.iowa.as_ref().unwrap_or(&0.0),
            12 => self.idaho.as_ref().unwrap_or(&0.0),
            13 => self.illinois.as_ref().unwrap_or(&0.0),
            14 => self.indiana.as_ref().unwrap_or(&0.0),
            15 => self.kansas.as_ref().unwrap_or(&0.0),
            16 => self.kentucky.as_ref().unwrap_or(&0.0),
            17 => self.louisiana.as_ref().unwrap_or(&0.0),
            18 => self.massachusetts.as_ref().unwrap_or(&0.0),
            19 => self.maryland.as_ref().unwrap_or(&0.0),
            20 => self.maine.as_ref().unwrap_or(&0.0),
            21 => self.michigan.as_ref().unwrap_or(&0.0),
            22 => self.minnesota.as_ref().unwrap_or(&0.0),
            23 => self.missouri.as_ref().unwrap_or(&0.0),
            24 => self.mississippi.as_ref().unwrap_or(&0.0),
            25 => self.montana.as_ref().unwrap_or(&0.0),
            26 => self.north_carolina.as_ref().unwrap_or(&0.0),
            27 => self.north_dakota.as_ref().unwrap_or(&0.0),
            28 => self.nebraska.as_ref().unwrap_or(&0.0),
            29 => self.new_hampshire.as_ref().unwrap_or(&0.0),
            30 => self.new_jersey.as_ref().unwrap_or(&0.0),
            31 => self.new_mexico.as_ref().unwrap_or(&0.0),
            32 => self.nevada.as_ref().unwrap_or(&0.0),
            33 => self.new_york.as_ref().unwrap_or(&0.0),
            34 => self.ohio.as_ref().unwrap_or(&0.0),
            35 => self.oklahoma.as_ref().unwrap_or(&0.0),
            36 => self.oregon.as_ref().unwrap_or(&0.0),
            37 => self.pennsylvania.as_ref().unwrap_or(&0.0),
            38 => self.rhode_island.as_ref().unwrap_or(&0.0),
            39 => self.south_carolina.as_ref().unwrap_or(&0.0),
            40 => self.south_dakota.as_ref().unwrap_or(&0.0),
            41 => self.tennessee.as_ref().unwrap_or(&0.0),
            42 => self.texas.as_ref().unwrap_or(&0.0),
            43 => self.utah.as_ref().unwrap_or(&0.0),
            44 => self.virginia.as_ref().unwrap_or(&0.0),
            45 => self.vermont.as_ref().unwrap_or(&0.0),
            46 => self.washington.as_ref().unwrap_or(&0.0),
            47 => self.wisconsin.as_ref().unwrap_or(&0.0),
            48 => self.west_virginia.as_ref().unwrap_or(&0.0),
            49 => self.wyoming.as_ref().unwrap_or(&0.0),
            _ => panic!("Invalid index"),
        }
    }
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
    let records = read_csv(UNEMPLOYMENT_PATH);
    let record = records
        .iter()
        .find(|record| record.date == date)
        .ok_or_else(|| format!("No record found for date: {}", date))?;
    let mut min = f32::MAX;
    let mut max = f32::MIN;
    for index in 0..50 {
        let value = record[index];
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
    }
    Ok((min, max))
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_record_index() {
        let records = read_csv(UNEMPLOYMENT_PATH);
        let _number = records[0][51];
    }

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

    #[test]
    fn test_get_date_min_max() {
        let some_date = NaiveDate::from_ymd_opt(1980, 1, 1).unwrap();
        let (min, max) = get_date_min_max(some_date).unwrap();
        assert_eq!(min, 2.9);
        assert_eq!(max, 9.6);
        assert!(get_date_min_max(NaiveDate::from_ymd_opt(1979, 1, 1).unwrap()).is_err());
    }
}
