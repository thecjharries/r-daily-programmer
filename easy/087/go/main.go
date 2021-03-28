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

type Coordinate struct {
	X float64
	Y float64
}

func NewCoordinate(x, y float64) *Coordinate {
	return &Coordinate{X: x, Y: y}
}

type Rectangle struct {
	TopLeft     *Coordinate
	BottomRight *Coordinate
}

func (r *Rectangle) ContainsRectangle(rectangle *Rectangle) bool {
	return r.TopLeft.X <= rectangle.TopLeft.X &&
		r.TopLeft.Y <= rectangle.TopLeft.Y &&
		r.BottomRight.X >= rectangle.BottomRight.X &&
		r.BottomRight.Y >= rectangle.BottomRight.Y
}

// Doesn't check validity of coordinates
func NewRectangleFromFloat64s(topLeftX, topLeftY, bottomRightX, bottomRightY float64) *Rectangle {
	return &Rectangle{
		TopLeft:     NewCoordinate(topLeftX, topLeftY),
		BottomRight: NewCoordinate(bottomRightX, bottomRightY),
	}
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func rectangleIntersection(first, second *Rectangle) (intersection *Rectangle) {
	if first.ContainsRectangle(second) {
		return second
	}
	if second.ContainsRectangle(first) {
		return first
	}
	return
}
