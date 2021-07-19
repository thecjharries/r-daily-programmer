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
	formula = fmt.Sprintf("%s%d%s", strings.Repeat("(", parenthesisCount), startingTerm, formula)
	expression, _ := govaluate.NewEvaluableExpression(formula)
	for index := 0; index < count; index++ {
		result, _ := expression.Eval(nil)
		sequence = append(sequence, result.(int))
	}
	return
}
