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

use std::io::Write;

fn main() {
    println!("rad");
}

fn mcarthy_91(start: i32, output: &mut impl Write) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcarthy_91() {
        let mut output: Vec<u8> = Vec::new();
        assert_eq!(mcarthy_91(99, &mut output), 91);
        assert_eq!(&output, b"M(M(110)) since 99 is equal to or less than 100\nM(100) since 110 is greater than 100\nM(M(111)) since 100 is equal to or less than 100\nM(101) since 111 is greater than 100\n91 since 101 is greater than 100\n");
        output.clear();
        assert_eq!(mcarthy_91(101, &mut output), 91);
        assert_eq!(&output, b"91 since 101 is greater than 100\n");
        output.clear();
        assert_eq!(mcarthy_91(91, &mut output), 91);
        assert_eq!(&output, b"M(M(102)) since 91 is equal to or less than 100\nM(92) since 102 is greater than 100\nM(M(103)) since 92 is equal to or less than 100\nM(93) since 103 is greater than 100\nM(M(104)) since 93 is equal to or less than 100\nM(94) since 104 is greater than 100\nM(M(105)) since 94 is equal to or less than 100\nM(95) since 105 is greater than 100\nM(M(106)) since 95 is equal to or less than 100\nM(96) since 106 is greater than 100\nM(M(107)) since 96 is equal to or less than 100\nM(97) since 107 is greater than 100\nM(M(108)) since 97 is equal to or less than 100\nM(98) since 108 is greater than 100\nM(M(109)) since 98 is equal to or less than 100\nM(99) since 109 is greater than 100\nM(M(110)) since 99 is equal to or less than 100\nM(100) since 110 is greater than 100\nM(M(111)) since 100 is equal to or less than 100\nM(101) since 111 is greater than 100\n91 since 101 is greater than 100\n");
        output.clear();
        assert_eq!(mcarthy_91(102, &mut output), 92);
        assert_eq!(&output, b"M(92) since 102 is greater than 100\n");
    }
}
