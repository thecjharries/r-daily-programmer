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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn print_calendar(year: usize, month: usize) -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_calendar() {
        let output = "+--------------------+
|      January       |
+--------------------+
|M |T |W |T |F |S |S |
+--------------------+
|  |  |  |  |  |  | 1|
| 2| 3| 4| 5| 6| 7| 8|
| 9|10|11|12|13|14|15|
|16|17|18|19|20|21|22|
|23|24|25|26|27|28|29|
|30|31|  |  |  |  |  |
+--------------------+"
            .to_string;
        assert_eq!(output, print_calendar(2012, 1));
    }
}
