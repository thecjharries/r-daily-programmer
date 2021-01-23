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

// These patterns come directly from the prompt
// They're much too prescriptive and I don't like them.
var allowedFormPatterns = []string{
	`^\s*\d{10}\s*$`,
	`^\s*\d{3}-\d{3}-\d{4}\s*$`,
	`^\s*\d{3}\.\d{3}\.\d{4}\s*$`,
	`^\s*\(\d{3}\)\s{0,1}\d{3}-\d{4}\s*$`,
	`^\s*\d{3}\-\d{4}\s*$`,
}

func main() {
	fmt.Println("See tests for number validation")
}

func compilePatterns(patterns []string) []regexp.Regexp {
	var allowedFormRegexp []regexp.Regexp
	for _, pattern := range patterns {
		allowedFormRegexp = append(allowedFormRegexp, *regexp.MustCompile(pattern))
	}
	return allowedFormRegexp
}

func validateNumber(phoneNumber string, allowedFormRegexp []regexp.Regexp) bool {
	for _, formRegexp := range allowedFormRegexp {
		if formRegexp.MatchString(phoneNumber) {
			fmt.Print(formRegexp.String())
			return true
		}
	}
	return false
}
