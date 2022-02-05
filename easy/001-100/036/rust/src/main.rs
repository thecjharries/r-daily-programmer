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

fn open_lockers(max: usize) -> Vec<bool> {
    let mut lockers = vec![false; max];
    for student in 1..max+1 {
        for locker in (student..max+1).step_by(student) {
            lockers[(locker-1)] = !lockers[(locker-1)];
        }
    }
    lockers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        let lockers = vec![true, false, false, true, false, false, false, false, true];
        assert_eq!(lockers, open_lockers(lockers.len()));
    }
}
