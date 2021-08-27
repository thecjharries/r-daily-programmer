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
	"strconv"
	"time"
)

var monthDayYearPattern = regexp.MustCompile(`^\s*(?P<month>\d{1,2})[^\d]+(?P<day>\d{1,2})[^\d]+(?P<year>\d{2}|\d{4})\s*$`)
var yearMonthDayPattern = regexp.MustCompile(`^\s*(?P<year>\d{4})[^\d]+(?P<month>\d{1,2})[^\d]+(?P<day>\d{1,2})\s*$`)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func parseDate(input string) string {
	var match []string
	var yearIndex, monthIndex, dayIndex int
	if monthDayYearPattern.MatchString(input) {
		match = monthDayYearPattern.FindStringSubmatch(input)
		yearIndex = 3
		monthIndex = 1
		dayIndex = 2
	} else {
		match = yearMonthDayPattern.FindStringSubmatch(input)
		yearIndex = 1
		monthIndex = 2
		dayIndex = 3
	}
	year, _ := strconv.Atoi(match[yearIndex])
	if 3 == yearIndex && 2 == len(match[yearIndex]) {
		year += 2000
	}
	month, _ := strconv.Atoi(match[monthIndex])
	day, _ := strconv.Atoi(match[dayIndex])
	return time.Date(year, time.Month(month), day, 0, 0, 0, 0, time.UTC).Format("2006-01-02")
}
