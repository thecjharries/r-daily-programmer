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

var conversionMap = map[string]string{
	"1":  "110",
	"0":  "100",
	"11": "1101",
	"01": "1001",
	"10": "1100",
	"00": "1000",
}

var cachedDragonSequenceTerms = map[int]string{
	1: "1",
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func generateDragonCurveTerm(index int) (result string) {
	if 1 > index {
		return cachedDragonSequenceTerms[1]
	}
	term, exists := cachedDragonSequenceTerms[index]
	if exists {
		return term
	}
	previousTerm := generateDragonCurveTerm(index - 1)
	for previousTermIndex := 0; previousTermIndex < len(previousTerm)/2; previousTermIndex++ {
		result += conversionMap[previousTerm[2*previousTermIndex:2*previousTermIndex+2]]
	}
	result += conversionMap[previousTerm[len(previousTerm)-1:]]
	cachedDragonSequenceTerms[index] = result
	return
}
