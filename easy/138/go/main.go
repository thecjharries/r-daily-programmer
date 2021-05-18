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

type Particle struct {
	Mass float64
	X    float64
	Y    float64
}

func (p *Particle) SimplifiedColombsLaw(secondParticle Particle) float64 {
	return math.Round((p.Mass*secondParticle.Mass)*1000/math.Sqrt(math.Pow(p.X-secondParticle.X, 2)+math.Pow(p.Y-secondParticle.Y, 2))) / 1000
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
