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

use std::collections::HashMap;

const ACCOUNT_INDEX: usize = 0;
const DEBIT_INDEX: usize = 2;
const CREDIT_INDEX: usize = 3;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn build_journal(input: &str) -> HashMap<String, i64> {
    let mut journal = HashMap::new();
    for line in input.lines() {
        let exploded = line.split(';').collect::<Vec<&str>>();
        let account = exploded[ACCOUNT_INDEX].to_string();
        let debit = exploded[DEBIT_INDEX].parse::<i64>().unwrap_or(0);
        let credit = exploded[CREDIT_INDEX].parse::<i64>().unwrap_or(0);
        let balance = journal.entry(account).or_insert(0);
        *balance += debit;
        *balance -= credit;
    }
    journal
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        let journal = build_journal("1000;JAN-16;100000;0;\n3000;JAN-16;0;100000;\n7140;JAN-16;36000;0;\n1000;JAN-16;0;36000;\n1100;FEB-16;80000;0;\n1000;FEB-16;0;60000;\n2000;FEB-16;0;20000;\n1110;FEB-16;17600;0;\n2010;FEB-16;0;17600;\n1000;MAR-16;28500;0;\n4000;MAR-16;0;28500;\n2010;MAR-16;17600;0;\n1000;MAR-16;0;17600;\n5000;APR-16;19100;0;\n1000;APR-16;0;19100;\n1000;APR-16;32900;0;\n1020;APR-16;21200;0;\n4000;APR-16;0;54100;\n1000;MAY-16;15300;0;\n1020;MAY-16;0;15300;\n1000;MAY-16;4000;0;\n4090;MAY-16;0;4000;\n1110;JUN-16;5200;0;\n2010;JUN-16;0;5200;\n5100;JUN-16;19100;0;\n1000;JUN-16;0;19100;\n4120;JUN-16;5000;0;\n1000;JUN-16;0;5000;\n7160;JUL-16;2470;0;\n2010;JUL-16;0;2470;\n5500;JUL-16;3470;0;\n1000;JUL-16;0;3470;");
        assert_eq!(20430, journal["1000"]);
    }
}
