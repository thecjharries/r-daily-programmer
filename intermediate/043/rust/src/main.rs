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
enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn doomsday_day_of_week(year: u32, month: u32, day: u32) -> Day {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doomsday_day_of_week() {
        assert_eq!(Day::Tuesday, doomsday_day_of_week(2012, 4, 24));
        assert_eq!(Day::Saturday, doomsday_day_of_week(2012, 4, 21));
        assert_eq!(Day::Saturday, doomsday_day_of_week(2000, 4, 1));
        assert_eq!(Day::Sunday, doomsday_day_of_week(2001, 4, 1));
        assert_eq!(Day::Friday, doomsday_day_of_week(2023, 2, 24));
    }
}
