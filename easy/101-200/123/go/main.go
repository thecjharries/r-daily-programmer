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

var newlinePattern = regexp.MustCompile(`\r?\n`)

const (
	windowsNewline = "\r\n"
	unixNewline    = "\n"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func fixNewlines(replacement, haystack string) string {
	return newlinePattern.ReplaceAllString(haystack, replacement)
}

func replaceNewlines(os, haystack string) string {
	switch os {
	case "windows":
		return fixNewlines(windowsNewline, haystack)
	case "posix":
		return fixNewlines(unixNewline, haystack)
	default:
		return haystack
	}
}
