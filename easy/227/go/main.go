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

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func convertPointToNumber(spiralSize, x, y int) (result int) {
	adjustedX := x - spiralSize/2 - 1
	adjustedY := y - spiralSize/2 - 1
	coefficient := int(math.Max(math.Abs(float64(adjustedX)), math.Abs(float64(adjustedY))))
	if coefficient == adjustedY {
		result = 4*coefficient*coefficient + 3*coefficient + 1 + adjustedX
	} else if -1*coefficient == adjustedY {
		result = 4*coefficient*coefficient - coefficient + 1 - adjustedX
	} else if coefficient == x {
		result = 4*coefficient*coefficient - 3*coefficient + 1 - adjustedY
	} else {
		result = 4*coefficient*coefficient + coefficient + 1 + adjustedY
	}
	return
}

func convertNumberToPoint(spiralSize, number int) (x, y int) {
	coefficient := 1 + (math.Sqrt(float64(number-1))-1)/2
	x = 1 + spiralSize/2 + int(math.Min(coefficient, math.Max(-coefficient, math.Abs(float64(number)-4*coefficient*coefficient-coefficient-1)-2*coefficient)))
	y = 1 + spiralSize/2 + int(math.Min(coefficient, math.Max(-coefficient, math.Abs(float64(number)-4*coefficient*coefficient+coefficient-1)-2*coefficient)))
	return
}
