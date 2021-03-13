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

var rulesMap = map[string]rune {
	"111": '0',
	"110": '1',
	"101": '1',
	"011": '1',
	"001": '1',
	"010": '1',
	"100": '0',
	"000": '0',
}

func main() {
	fmt.Println("hello world")
}

func applyRules(pattern []rune) (newPattern []rune) {
	newPattern = make([]rune, len(pattern))
	copy(newPattern, pattern)
	newRune, ok := rulesMap[string(newPattern)]
	if ok {
		fmt.Println("ok")
		newPattern[1] = newRune
	} else {
		fmt.Println(string(newPattern))
	}
	return
}
