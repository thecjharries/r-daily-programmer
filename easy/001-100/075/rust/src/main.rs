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

use regex::Regex;

fn main() {
    println!("rad");
}

fn poorly_parse_c(input: String) -> String {
    let exploded = input.split("=").collect::<Vec<&str>>();
    let mut result = String::from("float ");
    let lhs_pattern = Regex::new(r"(?P<name>[^(]+)\((?P<args>[^)]*)\)").unwrap();
    let captured = lhs_pattern.captures(exploded[0]).unwrap();
    result += &captured["name"];
    result += "(";
    let args = captured["args"]
        .split(",")
        .map(|arg| format!("float {}", arg))
        .collect::<Vec<String>>();
    result += &args.join(",");
    result += ")";
    result += "\n{\n    return ";
    result += &exploded[1].replace("abs", "fabsf");
    result += ";\n}";
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            poorly_parse_c("L0(x,y)=abs(x)+abs(y)".to_string()),
            "float L0(float x,float y)
{
    return fabsf(x)+fabsf(y);
}"
            .to_string()
        );
    }
}
