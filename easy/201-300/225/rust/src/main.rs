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
    static ref BOX_EDGE_PATTERN: Regex = Regex::new(r"\+-+\+").unwrap();
    static ref BOX_MIDDLE_PATTERN: Regex = Regex::new(r"\|[^|]+\|").unwrap();
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn decolumnize_text(input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decolumnize_text() {
        assert_eq!(
            "This is an example piece of text. This is an example piece of text. This is an example piece of text. This is an example piece of text. This is a sample for a challenge. Lorum ipsum dolor sit amet and other words. The proper word for a layout like this would be typesetting, or so I would imagine, but for now let's carry on calling it an example piece of text. Hold up - the end of the paragraph is approaching - notice the double line break for a paragraph.\n\nAnd so begins the start of the second paragraph but as you can see it's only marginally better than the other one so you've not really gained much - sorry. I am certainly not a budding author as you can see from this example input. Perhaps I need to work on my writing skills.",
            decolumnize_text(
                "This is an example piece of text. This is an exam-
ple piece of text. This is an example piece of
text. This is an example
piece of text. This is a +-----------------------+
sample for a challenge.  |                       |
Lorum ipsum dolor sit a- |       top class       |
met and other words. The |        feature        |
proper word for a layout |                       |
like this would be type- +-----------------------+
setting, or so I would
imagine, but for now let's carry on calling it an
example piece of text. Hold up - the end of the
                 paragraph is approaching - notice
+--------------+ the double line break for a para-
|              | graph.
|              |
|   feature    | And so begins the start of the
|   bonanza    | second paragraph but as you can
|              | see it's only marginally better
|              | than the other one so you've not
+--------------+ really gained much - sorry. I am
                 certainly not a budding author
as you can see from this example input. Perhaps I
need to work on my writing skills."
            )
        );
    }
}
