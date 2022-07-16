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

use evalexpr::{eval_int_with_context, ContextWithMutableVariables, HashMapContext, Value};

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn evaluate_recurrence_relation(relation: &str, start: i64, count: usize) -> Vec<i64> {
    let op_count = relation.matches(" ").count();
    let mut expression = relation.replace(" ", ")");
    expression = format!("{}x{}", "(".repeat(op_count), expression);
    let mut result = vec![start];
    let mut context = HashMapContext::new();
    for _ in 0..count {
        context.set_value("x".to_string(), Value::Int(result[result.len() - 1]));
        result.push(eval_int_with_context(&expression, &context).unwrap());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_recurrence_relation() {
        assert_eq!(
            vec![1, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047],
            evaluate_recurrence_relation("*2 +1", 1, 10)
        );
        assert_eq!(
            vec![0, 1, 4, 13, 40, 121, 364, 1093, 3280, 9841, 29524],
            evaluate_recurrence_relation("+2 *3 -5", 0, 10)
        );
        assert_eq!(
            vec![1, -2, 4, -8],
            evaluate_recurrence_relation("*(-2)", 1, 3)
        );
    }
}
