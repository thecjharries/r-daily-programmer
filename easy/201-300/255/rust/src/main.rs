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

fn run_switch_exercise(number_of_switches: usize, toggles: Vec<(usize, usize)>) -> usize {
    let mut switches = vec![false; number_of_switches];
    for (start, end) in toggles {
        let mut first = start;
        let mut last = end;
        if first > last {
            first = end;
            last = start;
        }
        for index in first..=last {
            switches[index] = !switches[index];
        }
    }
    switches.iter().filter(|&switch| *switch).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            7,
            run_switch_exercise(10, vec![(3, 6), (0, 4), (7, 3), (9, 9),])
        );
        assert_eq!(
            423,
            run_switch_exercise(
                1000,
                vec![
                    (616, 293),
                    (344, 942),
                    (27, 524),
                    (716, 291),
                    (860, 284),
                    (74, 928),
                    (970, 594),
                    (832, 772),
                    (343, 301),
                    (194, 882),
                    (948, 912),
                    (533, 654),
                    (242, 792),
                    (408, 34),
                    (162, 249),
                    (852, 693),
                    (526, 365),
                    (869, 303),
                    (7, 992),
                    (200, 487),
                    (961, 885),
                    (678, 828),
                    (441, 152),
                    (394, 453),
                ]
            )
        );
    }
}
