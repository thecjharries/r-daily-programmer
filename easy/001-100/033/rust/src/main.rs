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

struct Question {
    question: String,
    answer: String,
}

impl Question {
    fn new(question: String, answer: String) -> Self {
        Question { question, answer }
    }

    fn check(&self, guess: &str) -> bool {
        self.answer == guess
    }
}

fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_ctor() {
        let test = Question::new("What is the capital of France?".to_string(), "Paris".to_string());
        assert_eq!(test.question, "What is the capital of France?".to_string());
        assert_eq!(test.answer, "Paris".to_string());
    }
}
