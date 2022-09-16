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

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NOT_LEADING_CAPITAL_PATTERN: Regex = Regex::new(r"^[A-Z]+").unwrap();
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn indent_properly(input: &str, delimiter: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            "VAR I\nFOR I=1 TO 31\n····IF !(I MOD 3) THEN\n········PRINT \"FIZZ\"\n····ENDIF\n····IF !(I MOD 5) THEN\n········PRINT \"BUZZ\"\n····ENDIF\n····IF (I MOD 3) && (I MOD 5) THEN\n········PRINT \"FIZZBUZZ\"\n····ENDIF\nNEXT".to_string(),
            indent_properly(
                "VAR I\n·FOR I=1 TO 31\n»»»»IF !(I MOD 3) THEN\n··PRINT \"FIZZ\"\n··»»ENDIF\n»»»»····IF !(I MOD 5) THEN\n»»»»··PRINT \"BUZZ\"\n··»»»»»»ENDIF\n»»»»IF (I MOD 3) && (I MOD 5) THEN\n······PRINT \"FIZZBUZZ\"\n··»»ENDIF\n»»»»·NEXT",
                "····"
            )
        );
    }
}
