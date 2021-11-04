// Copyright 2021 CJ Harries
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

package main

import (
	"fmt"

	prmt "github.com/gitchander/permutation"
	"github.com/knetic/govaluate"
)

var operators = []string{"+", "-", "*", "-"}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func generateOperatorPossibilities(count int) [][]string {
	if count == 0 {
		return [][]string{[]string{}}
	}
	var result [][]string
	for _, op := range operators {
		for _, sub := range generateOperatorPossibilities(count - 1) {
			result = append(result, append([]string{op}, sub...))
		}
	}
	return result
}

func countdown(input []int) string {
	possibleOps := generateOperatorPossibilities(5)
	permInput := make([]int, len(input))
	copy(permInput, input)
	permutation := prmt.New(prmt.IntSlice(permInput))
	for permutation.Next() {
		for _, ops := range possibleOps {
			equation := fmt.Sprintf(
				"(((((%d %s %d) %s %d) %s %d) %s %d) %s %d) == %d",
				permInput[0],
				ops[0],
				permInput[1],
				ops[1],
				permInput[2],
				ops[2],
				permInput[3],
				ops[3],
				permInput[4],
				ops[4],
				permInput[5],
				permInput[6],
			)
			expression, _ := govaluate.NewEvaluableExpression(equation)
			result, _ := expression.Evaluate(nil)
			if result.(bool) {
				return fmt.Sprintf(
					"%d %s %d %s %d %s %d %s %d %s %d = %d",
					permInput[0],
					ops[0],
					permInput[1],
					ops[1],
					permInput[2],
					ops[2],
					permInput[3],
					ops[3],
					permInput[4],
					ops[4],
					permInput[5],
					permInput[6],
				)
			}
		}
	}
	return ""
}
