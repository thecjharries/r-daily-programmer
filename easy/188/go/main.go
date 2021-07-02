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

const (
	DateFormatOne   = "2006-01-02"
	DateFormatTwo   = "01/02/06"
	DateFormatThree = "01/06/02"
	DateFormatFour  = "02*01*2006"
	DateFormatFive  = "Jan 02, 06"
	DateFormatSix   = "Jan 02, 2006"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func parseDate(input, format string) time.Time {
	date, err := time.Parse(format, input)
	if nil != err {
		return time.Time{}
	}
	return date
}

func parseDates(dates []string) (parsedDates []time.Time) {
	for _, input := range dates {
		for _, format := range []string{DateFormatOne, DateFormatTwo, DateFormatThree, DateFormatFour, DateFormatFive, DateFormatSix} {
			newDate := parseDate(input, format)
			if "0001-01-01 00:00:00 +0000 UTC" != newDate.String() {
				parsedDates = append(parsedDates, newDate)
				break
			}
		}
	}
	return
}
