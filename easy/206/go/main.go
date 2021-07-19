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
	"strings"

	"github.com/knetic/govaluate"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func recurrenceRelation(relation string, startingTerm, count int) (sequence []int) {
	formula := strings.Replace(relation, " ", ")", -1)
	parenthesisCount := strings.Count(formula, ")")
	formula = fmt.Sprintf("%scurrent%s", strings.Repeat("(", parenthesisCount), formula)
	expression, _ := govaluate.NewEvaluableExpression(formula)
	parameters := make(map[string]interface{}, 8)
	parameters["current"] = startingTerm
	sequence = append(sequence, startingTerm)
	for index := 0; index < count; index++ {
		result, _ := expression.Evaluate(parameters)
		sequence = append(sequence, int(result.(float64)))
		parameters["current"] = int(result.(float64))
	}
	return
}
