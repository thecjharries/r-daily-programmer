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

import "fmt"

type ReversePolishNotationEntry interface{}

type ReversePolishNotation []ReversePolishNotationEntry

func multiply(a, b int) int {
	return a * b
}

func add(a, b int) int {
	return a + b
}

func subtract(a, b int) int {
	return a - b
}

type ReversePolishNotationOperator func(int, int) int

var operators = map[string]ReversePolishNotationOperator{
	"*": multiply,
	"+": add,
	"-": subtract,
}

func main() {
	fmt.Println(computeReversePolishNotation(ReversePolishNotation{3, 5, "+", 7, 2, "-", "*"}))
}

func computeReversePolishNotation(notation ReversePolishNotation) int {
	var filo []int
	filo = append(filo, notation[0].(int), notation[1].(int))
	for index := 2; index < len(notation); index++ {
		entry, isString := notation[index].(string)
		if isString {
			operation, allowedOperation := operators[entry]
			if allowedOperation {
				filo = append(filo[:len(filo)-2], operation(filo[len(filo)-2], filo[len(filo)-1]))
			}
		} else {
			filo = append(filo, notation[index].(int))
		}
	}
	return filo[0]
}
