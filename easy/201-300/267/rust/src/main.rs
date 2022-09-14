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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn build_places_not_won(place_won: u32, number_of_places: u32) -> String {
    let mut places: Vec<String> = Vec::new();
    for place in 0..=number_of_places {
        if place != place_won {
            let place_string = place.to_string();
            if place_string.ends_with("1") {
                places.push(format!("{}st", place_string));
            } else if place_string.ends_with("2") {
                places.push(format!("{}nd", place_string));
            } else if place_string.ends_with("3") {
                places.push(format!("{}rd", place_string));
            } else {
                places.push(format!("{}th", place_string));
            }
        }
    }
    places.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_places_not_won() {
        assert_eq!("0th, 2nd, 3rd, 4th, 5th, 6th, 7th, 8th, 9th, 10th, 11st, 12nd, 13rd, 14th, 15th, 16th, 17th, 18th, 19th, 20th, 21st, 22nd, 23rd, 24th, 25th, 26th, 27th, 28th, 29th, 30th, 31st, 32nd, 33rd, 34th, 35th, 36th, 37th, 38th, 39th, 40th, 41st, 42nd, 43rd, 44th, 45th, 46th, 47th, 48th, 49th, 50th, 51st, 52nd, 53rd, 54th, 55th, 56th, 57th, 58th, 59th, 60th, 61st, 62nd, 63rd, 64th, 65th, 66th, 67th, 68th, 69th, 70th, 71st, 72nd, 73rd, 74th, 75th, 76th, 77th, 78th, 79th, 80th, 81st, 82nd, 83rd, 84th, 85th, 86th, 87th, 88th, 89th, 90th, 91st, 92nd, 93rd, 94th, 95th, 96th, 97th, 98th, 99th, 100th".to_string(), build_places_not_won(1, 100));
    }
}
