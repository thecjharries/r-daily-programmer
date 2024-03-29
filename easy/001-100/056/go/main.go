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
	"os"
)

const romanAlphabet string = "abcdefghijklmnopqrstuvwxyz"

func main() {
	writeAbacabaSequenceToFile(romanAlphabet, "output.txt")
}

func deleteFile(filename string) {
	_ = os.Remove(filename)
}

func writeAbacabaSequenceToFile(alphabet, filename string) {
	deleteFile(filename)
	fileHandle, _ := os.Create(filename)
	defer (func() { _ = fileHandle.Close() })()
	var abacabaCharacters []rune
	for _, currentCharacter := range alphabet {
		_, _ = fileHandle.WriteString(
			fmt.Sprintf(
				"%s%s%s\n",
				string(abacabaCharacters),
				string(currentCharacter),
				string(abacabaCharacters),
			),
		)
		abacabaCharacters = append(abacabaCharacters, append([]rune{currentCharacter}, abacabaCharacters...)...)
	}
}
