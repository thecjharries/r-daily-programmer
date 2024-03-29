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

fn determine_politeness(number: u32) -> u32 {
    let mut count = 0;
    for divisor in (3..number + 1).step_by(2) {
        if number % divisor == 0 {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(determine_politeness(0), 0);
        assert_eq!(determine_politeness(1), 0);
        assert_eq!(determine_politeness(2), 0);
        assert_eq!(determine_politeness(3), 1);
        assert_eq!(determine_politeness(4), 0);
        assert_eq!(determine_politeness(5), 1);
        assert_eq!(determine_politeness(7), 1);
        assert_eq!(determine_politeness(8), 0);
        assert_eq!(determine_politeness(9), 2);
        assert_eq!(determine_politeness(10), 1);
        assert_eq!(determine_politeness(11), 1);
        assert_eq!(determine_politeness(12), 1);
        assert_eq!(determine_politeness(13), 1);
        assert_eq!(determine_politeness(14), 1);
        assert_eq!(determine_politeness(15), 3);
        assert_eq!(determine_politeness(16), 0);
    }
}
