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

fn find_sad_cycle(base: u32, start: u64) -> Vec<u64> {
    let mut cycle = Vec::new();
    let mut previous = 0;
    let mut current = start;
    let mut in_cycle = false;
    loop {
        let mut next = 0;
        while current > 0 {
            next += (current % 10).pow(base);
            current /= 10;
        }
        if previous == next {
            return vec![next];
        }
        previous = next;
        current = next;
        if cycle.contains(&next) {
            if in_cycle {
                let index = cycle.iter().position(|&x| x == next).unwrap();
                return cycle[index - 1..].to_vec();
            } else {
                in_cycle = true;
                continue;
            }
        }
        cycle.push(next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_sad_cycle() {
        assert_eq!(
            vec![383890, 1057187, 513069, 594452, 570947, 786460, 477201, 239459, 1083396, 841700],
            find_sad_cycle(6, 2)
        );
        assert_eq!(
            vec![5345158, 2350099, 9646378, 8282107, 5018104, 2191663],
            find_sad_cycle(7, 7)
        );
        assert_eq!(vec![371], find_sad_cycle(3, 14));
        assert_eq!(vec![89, 145, 42, 20, 4, 16, 37, 58], find_sad_cycle(2, 12));
    }
}
