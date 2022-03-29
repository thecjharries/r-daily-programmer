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

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("rad");
}

fn load_data(filename: &str) -> Vec<f64> {
    let file = File::open(filename).expect("file not found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("error reading line"))
        .map(|line| line.parse::<f64>().expect("error parsing number"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_data() {
        assert_eq!(
            load_data("../dataset.txt"),
            vec![
                0.4081, 0.5514, 0.0901, 0.4637, 0.5288, 0.0831, 0.0054, 0.0292, 0.0548, 0.446,
                0.0009, 0.9525, 0.2079, 0.3698, 0.4966, 0.0786, 0.4684, 0.1731, 0.1008, 0.3169,
                0.022, 0.1763, 0.5901, 0.4661, 0.652, 0.1485, 0.0049, 0.7865, 0.8373, 0.6934,
                0.3973, 0.3616, 0.4538, 0.2674, 0.3204, 0.5798, 0.2661, 0.0799, 0.0132, 0.0,
                0.1827, 0.2162, 0.9927, 0.1966, 0.1793, 0.7147, 0.3386, 0.2734, 0.5966, 0.9083,
                0.3049, 0.0711, 0.0142, 0.1799, 0.318, 0.6281, 0.0073, 0.265, 0.0008, 0.4552
            ]
        );
    }
}
