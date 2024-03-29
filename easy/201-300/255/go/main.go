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

type Switches map[int]bool

func NewSwitches(count int) (switches Switches) {
	switches = make(Switches, count)
	for index := 0; index < count; index++ {
		switches[index] = false
	}
	return switches
}

func (s *Switches) Toggle(min, max int) {
	left, right := min, max
	if min > max {
		left, right = max, min
	}
	for index := left; index <= right; index++ {
		(*s)[index] = !(*s)[index]
	}
}

func (s *Switches) GetOnCount() (total int) {
	for _, value := range *s {
		if value {
			total++
		}
	}
	return
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
