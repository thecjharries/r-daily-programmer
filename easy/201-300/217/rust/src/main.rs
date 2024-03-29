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

fn distribute_logs(new_logs: u32, log_piles: Vec<u32>) -> Vec<u32> {
    let mut final_distribution = log_piles.clone();
    let mut remaining_logs = new_logs;
    let mut current_smallest_logs = 0;
    while 0 < remaining_logs {
        let mut next_smallest_logs = u32::MAX;
        for index in 0..final_distribution.len() {
            if current_smallest_logs == final_distribution[index] {
                final_distribution[index] += 1;
                remaining_logs -= 1;
            }
            if 0 == remaining_logs {
                break;
            }
            if final_distribution[index] < next_smallest_logs {
                next_smallest_logs = final_distribution[index];
            }
        }
        current_smallest_logs = next_smallest_logs;
    }
    final_distribution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(vec![42], distribute_logs(41, vec![1]));
        assert_eq!(
            vec![3, 2, 2, 2, 2, 3, 2, 4, 2],
            distribute_logs(7, vec![1, 1, 1, 2, 1, 3, 1, 4, 1])
        );
    }
}
