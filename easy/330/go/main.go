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

type Circle struct {
	CenterX float64
	CenterY float64
	Radius  float64
}

type Rectangle struct {
	MinX float64
	MaxX float64
	MinY float64
	MaxY float64
}

func NewExtremeRectangle() *Rectangle {
	return &Rectangle{
		MinX: math.Inf(1),
		MaxX: math.Inf(-1),
		MinY: math.Inf(1),
		MaxY: math.Inf(-1),
	}
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
