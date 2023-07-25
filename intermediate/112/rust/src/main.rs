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

use chrono::NaiveDateTime;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn sort_dates(input: Vec<String>) -> Vec<String> {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_dates_should_properly_sort_prompt_input() {
        // let input = "2012 12 02 23:02:12
        // 1899 03 02 14:04:42
        // 1969 07 20 02:25:30
        // 2019 11 02 00:13:01";
        let input = vec![
            "2012 12 02 23:02:12".to_string(),
            "1899 03 02 14:04:42".to_string(),
            "1969 07 20 02:25:30".to_string(),
            "2019 11 02 00:13:01".to_string(),
        ];
        let output = vec![
            "1899 03 02 14:04:42".to_string(),
            "1969 07 20 02:25:30".to_string(),
            "2012 12 02 23:02:12".to_string(),
            "2019 11 02 00:13:01".to_string(),
        ];
        assert_eq!(output, sort_dates(input));
    }
}
