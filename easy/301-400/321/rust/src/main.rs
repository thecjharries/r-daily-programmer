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

fn print_time(time: &str) -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!("It's twelve am".to_string(), print_time("00:00"));
        assert_eq!("It's one thirty am".to_string(), print_time("01:30"));
        assert_eq!("It's twelve oh five pm".to_string(), print_time("12:05"));
        assert_eq!("It's two oh one pm".to_string(), print_time("14:01"));
        assert_eq!("It's eight twenty nine pm".to_string(), print_time("20:29"));
        assert_eq!("It's nine pm".to_string(), print_time("21:00"));
    }
}
