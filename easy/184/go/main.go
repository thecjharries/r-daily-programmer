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

type SmartStack struct {
	Sorted, Stack []int
}

func (s *SmartStack) Push(element int) {
	s.Stack = append(s.Stack, element)
	for index := 0; index < len(s.Sorted); index++ {
		if element < s.Sorted[index] {
			s.Sorted = append(s.Sorted[:index], append([]int{element}, s.Sorted[index:]...)...)
			break
		}
	}
	if len(s.Sorted) != len(s.Stack) {
		s.Sorted = append(s.Sorted, element)
	}
}

func (s *SmartStack) Pop() (element int) {
	element = s.Stack[len(s.Stack)-1]
	s.Stack = s.Stack[:len(s.Stack)-1]
	for index := 0; index < len(s.Sorted); index++ {
		if element == s.Sorted[index] {
			s.Sorted = append(s.Sorted[:index], s.Sorted[index+1:]...)
			break
		}
	}
	return element
}

func (s *SmartStack) Len() int {
	return len(s.Stack)
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
