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

fn main() {
    game_loop();
}

fn game_loop() {
    let mut north_south = 0;
    let mut east_west = 0;
    loop {
        let mut input = String::new();
        println!("Action? (n/s/e/w/q)");
        std::io::stdin().read_line(&mut input).unwrap();
        match input.as_str() {
            "n\n" => north_south += 1,
            "s\n" => north_south -= 1,
            "e\n" => east_west += 1,
            "w\n" => east_west -= 1,
            "q\n" => return,
            _ => println!("Invalid input"),
        }
        println!("You are at {} {}", north_south, east_west);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
