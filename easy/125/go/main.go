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
	"regexp"
)

var symbolPattern = regexp.MustCompile(`(?i)[^a-z0-9\s]`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func countSymbols(haystack string) (count int) {
	matches := symbolPattern.FindAllStringIndex(haystack, -1)
	fmt.Println(matches)
	return len(matches)
}
