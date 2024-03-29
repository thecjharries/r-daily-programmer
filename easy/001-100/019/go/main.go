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
	"io/ioutil"
	"path"
	"regexp"
)

var pathToText string = path.Join("..", "project-gutenberg_the-adventures-of-sherlock-holmes.txt")
var headerPattern *regexp.Regexp = regexp.MustCompile(`(?ms).+?^\*\*\* START OF.+?$`)
var footerPattern *regexp.Regexp = regexp.MustCompile(`(?ms)\*\*\* END OF.+`)
var allowedCharactersPattern = regexp.MustCompile(`(?ims)[a-z0-9]`)

func main() {
	haystack := loadFileIntoString(pathToText)
	haystack = stripPatternFromString(headerPattern, haystack)
	haystack = stripPatternFromString(footerPattern, haystack)
	fmt.Println(haystack)
	fmt.Printf("Characters in Sherlock Holmes: %d", countAllowedCharacters(allowedCharactersPattern, haystack))
}

func loadFileIntoString(pathToFile string) string {
	content, _ := ioutil.ReadFile(pathToFile)
	return string(content)
}

func stripPatternFromString(patternToRemove *regexp.Regexp, haystack string) string {
	return patternToRemove.ReplaceAllString(haystack, "")
}

func countAllowedCharacters(patternForAllowedCharacters *regexp.Regexp, haystack string) int {
	return len(patternForAllowedCharacters.FindAllStringIndex(haystack, -1))
}
