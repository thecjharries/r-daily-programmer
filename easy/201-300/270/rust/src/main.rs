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

fn transpose_text(input: &str) -> String {
    let lines = input.split("\n").collect::<Vec<&str>>();
    println!("{:?}", lines);
    let mut output = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        while output.len() < line.len() {
            output.push(String::from(" ".repeat(index)));
        }
        println!("{:?}", output);
        println!("{} {}", index, line);
        for (character_index, character) in line.chars().enumerate() {
            output[character_index].push(character);
        }
        for character_index in line.len()..output.len() {
            output[character_index].push(' ');
        }
    }
    output
        .into_iter()
        .map(|line| line.trim_end().to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose_text() {
        assert_eq!(
            "St\noe\nmx\net\n .".to_string(),
            transpose_text("Some\ntext.")
        );
        assert_eq!(
            "p i f       }\na m u\nc p n\nk o c\na r  qqqcf }\ng t muuulo\ne   aeeeor\n  \" iuuus\nm f neeeeef\na m (   (lm\ni t ):<<qet\nn \"  =--um.\n    {   e P\n     m\"\"u:r\n     aote=i\n     knw) n\n     eeo rt\n     (\"O al\n     c \" nn\n     h   g(\n     a   ee\n     n    l\n         qe\n     s   um\n     t   e)\n     r   u\n     i   e\n     n\n     g   {\n     ,\n\n     2\n     )" .to_string(),
            transpose_text("package main\n\nimport \"fmt\"\n\nfunc main() {\n    queue := make(chan string, 2)\n    queue <- \"one\"\n    queue <- \"twoO\"\n    close(queue)\n    for elem := range queue {\n        fmt.Println(elem)\n    }\n}")
        )
    }
}
