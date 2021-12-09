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

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func check(word string) bool {
	for index := 1; index < len(word)-2; index++ {
		if 'c' == word[index-1] && 'i' == word[index] && 'e' == word[index+1] {
			return false
		}
		if 'c' != word[index-1] && 'e' == word[index] && 'i' == word[index+1] {
			return false
		}
	}
	return true
}
