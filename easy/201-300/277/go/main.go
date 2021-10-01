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

var zPrint = fmt.Println

type Fraction struct {
	Numerator   int
	Denominator int
}

func NewFraction(numerator, denominator int) *Fraction {
	return &Fraction{
		Numerator:   numerator,
		Denominator: denominator,
	}
}

func (f *Fraction) Reduce() *Fraction {
	a := f.Numerator
	b := f.Denominator
	if a < b {
		a = f.Denominator
		b = f.Numerator
	}
	for 0 < a%b {
		fmt.Println(a, b, a%b)
		a, b = b, a%b
	}
	f.Numerator /= b
	f.Denominator /= b
	return f
}

func main() {
	_, _ = zPrint("hello world")
}
