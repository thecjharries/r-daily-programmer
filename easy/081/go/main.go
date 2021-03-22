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

type R2Point struct {
	X float64
	Y float64
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint(discreteSlopes([]R2Point{{-1,-1},{-0.5,-0.5},{0,0},{0.5,0.5},{1,1}}))
}

func discreteSlopes(coordinates []R2Point) (slopes []float64) {
	if len(coordinates) < 2 {
		return
	}
	slopes = make([]float64, len(coordinates) - 1)
	for index := 0; index < len(coordinates) - 1; index++ {
		slopes[index] = (coordinates[index + 1].Y - coordinates[index].Y) / (coordinates[index + 1].X - coordinates[index].X)
	}
	return
}
