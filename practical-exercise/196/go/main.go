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

type IntegerSet map[int]struct{}

func (s *IntegerSet) Contains(number int) bool {
	_, exists := (*s)[number]
	return exists
}

func (s *IntegerSet) Union(otherSet IntegerSet) IntegerSet {
	set := make(IntegerSet)
	for key, _ := range *s {
		set[key] = struct{}{}
	}
	for key, _ := range otherSet {
		set[key] = struct{}{}
	}
	return set
}

func (s *IntegerSet) Intersection(otherSet IntegerSet) IntegerSet {
	set := make(IntegerSet)
	for key, _ := range *s {
		if _, exists := otherSet[key]; exists {
			set[key] = struct{}{}
		}
	}
	return set
}

func (s *IntegerSet) Equals(otherSet IntegerSet) bool {
	if len(otherSet) != len(*s) {
		return false
	}
	for key, _ := range *s {
		if _, exists := otherSet[key]; !exists {
			return false
		}
	}
	return true
}

func NewIntegerSet(numbers ...int) IntegerSet {
	set := make(IntegerSet)
	for _, number := range numbers {
		set[number] = struct{}{}
	}
	return set
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
