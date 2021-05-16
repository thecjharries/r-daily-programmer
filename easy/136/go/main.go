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
	"math"
)

type Student struct {
	Name   string
	Grades []float64
}

func (s *Student) GetAverageFraction() (float64, float64) {
	sum := float64(0)
	for _, score := range s.Grades {
		sum += score
	}
	return sum, float64(len(s.Grades))
}

func (s *Student) GetAverage() float64 {
	sum, total := s.GetAverageFraction()
	return math.Round(sum*100/total) / 100
}

func (s *Student) String() string {
	return fmt.Sprintf("%s %.2f", s.Name, s.GetAverage())
}

func NewStudent(name string, grades []float64) *Student {
	return &Student{
		Name:   name,
		Grades: grades,
	}
}

type Class []Student

func (c *Class) GetAverage() float64 {
	sum, total := float64(0), float64(0)
	for _, student := range *c {
		studentSum, studentTotal := student.GetAverageFraction()
		sum += studentSum
		total += studentTotal
	}
	return math.Round(sum*100/total) / 100
}

func (c *Class) String() (output string) {
	output = fmt.Sprintf("%.2f\n", c.GetAverage())
	for _, student := range *c {
		output += fmt.Sprintf("%s\n", student.String())
	}
	return
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
