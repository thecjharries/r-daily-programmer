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

const GravitationalConstant float64 = 6.67e-11

type Planet struct {
	Name           string
	Radius         float64
	AverageDensity float64
}

func (p *Planet) Volume() float64 {
	return 4.0 * math.Pi * math.Pow(p.Radius, 3) / 3
}

func (p *Planet) Mass() float64 {
	return p.Volume() * p.AverageDensity
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
