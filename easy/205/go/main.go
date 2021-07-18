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
	"time"
)

var inputDateFormat = "2006-01-02"

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func simplifyDateRange(first, second string) string {
	firstDate, _ := time.Parse(inputDateFormat, first)
	secondDate, _ := time.Parse(inputDateFormat, second)
	if secondDate.Year() == firstDate.Year() {
		if secondDate.Month() == firstDate.Month() {
			return fmt.Sprintf("%s - %d", firstDate.Format("January 2"), secondDate.Day())
		} else {
			return fmt.Sprintf("%s - %s", firstDate.Format("January 2"), secondDate.Format("January 2"))
		}
	}
	return fmt.Sprintf("%s - %s", firstDate.Format("January 2, 2006"), secondDate.Format("January 2, 2006"))
}
