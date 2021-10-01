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
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func validateChemicalSymbol(element, symbol string) bool {
	sanitizedElement := strings.ToLower(element)
	sanitizedSymbol := strings.ToLower(symbol)
	symbolIndex := 0
	for elementIndex, _ := range sanitizedElement {
		if sanitizedElement[elementIndex:elementIndex+1] == sanitizedSymbol[symbolIndex:symbolIndex+1] {
			symbolIndex++
		}
		if 1 < symbolIndex {
			break
		}
	}
	return 1 < symbolIndex
}
