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
	"math/rand"

	combinations "github.com/mxschmitt/golang-combinations"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func createLottoLists(names []string, sizeOfList int) (output map[string][]string) {
	output = make(map[string][]string)
	shuffledNames := make([]string, len(names))
	copy(shuffledNames, names)
	rand.Shuffle(len(shuffledNames), func(i, j int) { shuffledNames[i], shuffledNames[j] = shuffledNames[j], shuffledNames[i] })
	possibleLists := combinations.Combinations(shuffledNames, sizeOfList)
	for index, name := range names {
		output[name] = possibleLists[index]
	}
	return
}
