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

use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant};

fn main() {
    println!("rad");
}

fn sleep_sort(numbers: Vec<i32>) -> Vec<i32> {
    let result = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for number in numbers {
        let result = Arc::clone(&result);
        let handle = spawn(move || {
            sleep(Duration::from_secs(number as u64));
            let mut result = result.lock().unwrap();
            result.push(number);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sleep_sort() {
        let total_duration = Duration::from_secs(10);
        let now = Instant::now();
        let result = sleep_sort(vec![5, 4, 3, 2, 6, 7, 8, 10, 9, 1]);
        assert!(now.elapsed() >= total_duration);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
