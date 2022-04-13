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
    println!("rad");
}

fn run_simulation(total_days: u32) -> u32 {
    let mut operational_days = 0;
    for current_day in 1..total_days + 1 {
        if 0 != current_day % 3 && 0 != current_day % 14 && 0 != current_day % 100 {
            operational_days += 1;
        }
    }
    operational_days
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_simulation() {
        assert_eq!(run_simulation(10), 7);
        assert_eq!(run_simulation(5991), 3675);
    }
}
