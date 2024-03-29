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

fn reverse_text(input: &str) -> String {
    let mut output = String::new();
    for line in input.split("\n") {
        let mut words = line.split(" ").collect::<Vec<_>>();
        words.reverse();
        output = format!("{}\n{}", &words.join(" "), output);
    }
    output.pop();
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_text() {
        assert_eq!(reverse_text(""), "");
        assert_eq!(
            reverse_text(
                "Tyger! Tyger! burning bright
In the forests of the night,
What immortal hand or eye
Could frame thy fearful symmetry?

In what distant deeps or skies
Burnt the fire of thine eyes?
On what wings dare he aspire?
What the hand dare sieze the fire?"
            ),
            "fire? the sieze dare hand the What
aspire? he dare wings what On
eyes? thine of fire the Burnt
skies or deeps distant what In

symmetry? fearful thy frame Could
eye or hand immortal What
night, the of forests the In
bright burning Tyger! Tyger!"
        )
    }
}
