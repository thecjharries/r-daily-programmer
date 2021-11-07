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
	"strconv"
	"strings"
)

var digits = []string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}
var teens = []string{"ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"}
var tens = []string{"ten", "twenty", "thirty", "forty", "fifty"}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func timeToWords(time string) (words string) {
	exploded := strings.Split(time, ":")
	hour, _ := strconv.Atoi(exploded[0])
	ampm := "am"
	if 11 < hour {
		hour -= 12
		ampm = "pm"
	}
	minute, _ := strconv.Atoi(exploded[1])
	var minuteWord string
	fmt.Println(hour, minute)
	if 0 == minute {
		minuteWord = ""
	} else if 10 > minute {
		minuteWord = fmt.Sprintf("oh %s ", digits[minute-1])
	} else if 20 > minute {
		minuteWord = fmt.Sprintf("%s ", teens[minute-10])
	} else {
		minuteWord = fmt.Sprintf("%s ", tens[minute/10-1])
		if 0 != minute%10 {
			minuteWord += fmt.Sprintf("%s ", digits[minute%10-1])
		}
	}
	return fmt.Sprintf(
		"It's %s %s%s",
		append(append([]string{"twelve"}, digits...), teens...)[hour],
		minuteWord,
		ampm,
	)
}
