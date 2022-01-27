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

use time::util::is_leap_year;

fn main() {
    println!("rad");
}

fn determine_year_data(year: i32) -> (i32, bool) {
    (((year as f32 / 100.0).ceil() as i32), is_leap_year(year))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_year_data() {
        assert_eq!(determine_year_data(2000), (20, true));
        assert_eq!(determine_year_data(1996), (20, true));
        assert_eq!(determine_year_data(1900), (19, true));
    }
}
