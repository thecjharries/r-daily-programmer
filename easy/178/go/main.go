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

type Point2d struct {
	X, Y float64
}

func NewPoint2d(x, y float64) *Point2d {
	return &Point2d{
		X: x,
		Y: y,
	}
}

func (p *Point2d) Translate(x, y float64) *Point2d {
	p.X += x
	p.Y += y
	return p
}

func (p *Point2d) Scale(x, y, factor float64) *Point2d {
	p.X = factor*(p.X-x) + x
	p.Y = factor*(p.Y-y) + y
	return p
}

func (p *Point2d) Rotate(x, y, theta float64) *Point2d {
	p.X, p.Y = math.Cos(-theta)*(p.X-x)-math.Sin(-theta)*(p.Y-y)+x, math.Sin(-theta)*(p.X-x)+math.Cos(-theta)*(p.Y-y)+y
	return p
}

func (p *Point2d) Reflect(xAxis, yAxis bool) *Point2d {
	if xAxis {
		p.Y *= -1
	}
	if yAxis {
		p.X *= -1
	}
	return p
}

func (p *Point2d) String() string {
	return fmt.Sprintf("(%f, %f)", p.X, p.Y)
}

var zPrint = fmt.Println

func main() {
	point := NewPoint2d(0, 5).
		Translate(3, 2).
		Scale(1, 3, 0.5).
		Rotate(3, 2, math.Pi/2).
		Reflect(true, false).
		Translate(2, -1).
		Scale(0, 0, -0.25).
		Rotate(1, -3, math.Pi).
		Reflect(false, true)
	_, _ = zPrint(point)
}
