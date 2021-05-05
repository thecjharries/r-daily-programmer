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
	"time"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func formatTime(format string, timeToFormat time.Time) (result string) {
	result = format
	result = strings.ReplaceAll(result, "%l", strconv.Itoa(timeToFormat.Nanosecond()/1000000))
	result = strings.ReplaceAll(result, "%s", strconv.Itoa(timeToFormat.Second()))
	result = strings.ReplaceAll(result, "%m", strconv.Itoa(timeToFormat.Minute()))
	result = strings.ReplaceAll(result, "%h", timeToFormat.Format("3"))
	result = strings.ReplaceAll(result, "%H", strconv.Itoa(timeToFormat.Hour()))
	amOrPm := "AM"
	if 11 < timeToFormat.Hour() {
		amOrPm = "PM"
	}
	result = strings.ReplaceAll(result, "%c", amOrPm)
	result = strings.ReplaceAll(result, "%d", strconv.Itoa(timeToFormat.Day()))
	result = strings.ReplaceAll(result, "%M", strconv.Itoa(int(timeToFormat.Month())))
	result = strings.ReplaceAll(result, "%y", strconv.Itoa(timeToFormat.Year()))
	return
}
