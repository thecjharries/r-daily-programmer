// Copyright 2023 CJ Harries
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

fn t9_to_letter(input: &str) -> Option<char> {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t9_to_letter_can_convert_letters() {
        assert_eq!(Some('a'), t9_to_letter("2"));
        assert_eq!(Some('b'), t9_to_letter("22"));
        assert_eq!(Some('c'), t9_to_letter("222"));
        assert_eq!(Some('d'), t9_to_letter("3"));
        assert_eq!(Some('e'), t9_to_letter("33"));
        assert_eq!(Some('f'), t9_to_letter("333"));
        assert_eq!(Some('g'), t9_to_letter("4"));
        assert_eq!(Some('h'), t9_to_letter("44"));
        assert_eq!(Some('i'), t9_to_letter("444"));
        assert_eq!(Some('j'), t9_to_letter("5"));
        assert_eq!(Some('k'), t9_to_letter("55"));
        assert_eq!(Some('l'), t9_to_letter("555"));
        assert_eq!(Some('m'), t9_to_letter("6"));
        assert_eq!(Some('n'), t9_to_letter("66"));
        assert_eq!(Some('o'), t9_to_letter("666"));
        assert_eq!(Some('p'), t9_to_letter("7"));
        assert_eq!(Some('q'), t9_to_letter("77"));
        assert_eq!(Some('r'), t9_to_letter("777"));
        assert_eq!(Some('s'), t9_to_letter("7777"));
        assert_eq!(Some('t'), t9_to_letter("8"));
        assert_eq!(Some('u'), t9_to_letter("88"));
        assert_eq!(Some('v'), t9_to_letter("888"));
        assert_eq!(Some('w'), t9_to_letter("9"));
        assert_eq!(Some('x'), t9_to_letter("99"));
        assert_eq!(Some('y'), t9_to_letter("999"));
        assert_eq!(Some('z'), t9_to_letter("9999"));
    }

    #[test]
    fn t9_to_letter_returns_none_for_invalid_input() {
        assert_eq!(None, t9_to_letter("1"));
        assert_eq!(None, t9_to_letter("11"));
        assert_eq!(None, t9_to_letter("111"));
        assert_eq!(None, t9_to_letter("1111"));
        assert_eq!(None, t9_to_letter("0"));
        assert_eq!(None, t9_to_letter("00"));
        assert_eq!(None, t9_to_letter("000"));
        assert_eq!(None, t9_to_letter("0000"));
        assert_eq!(None, t9_to_letter("a"));
        assert_eq!(None, t9_to_letter("aa"));
        assert_eq!(None, t9_to_letter("aaa"));
        assert_eq!(None, t9_to_letter("aaaa"));
        assert_eq!(None, t9_to_letter("b"));
        assert_eq!(None, t9_to_letter("bb"));
        assert_eq!(None, t9_to_letter("bbb"));
        assert_eq!(None, t9_to_letter("bbbb"));
        assert_eq!(None, t9_to_letter("c"));
        assert_eq!(None, t9_to_letter("cc"));
        assert_eq!(None, t9_to_letter("ccc"));
        assert_eq!(None, t9_to_letter("cccc"));
    }
}
