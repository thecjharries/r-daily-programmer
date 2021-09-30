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

	"github.com/tejainece/uu"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func uuencode(input string) (output string) {
	inputAsBytes := []byte(input)
	index := 0
	for index < len(inputAsBytes)-45 {
		output += string(uu.EncodeLine(inputAsBytes[index : index+45]))
		index += 45
	}
	output += string(uu.EncodeLine(inputAsBytes[index-45:]))
	return
}
