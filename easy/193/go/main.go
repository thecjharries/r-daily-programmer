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

func calculateCubeFromVolume(volume float64) string {
	edge := math.Pow(volume, 1.0/3)
	return fmt.Sprintf(
		// Note the fmt string is copied from the prompt with its comma splice
		"Cube: %0.2fm width, %0.2fm, high, %0.2fm tall",
		edge,
		edge,
		edge,
	)
}

func calculateCylinderFromVolume(volume float64) string {
	height := math.Pow(volume, 1.0/3)
	return fmt.Sprintf(
		"Cylinder: %0.2fm tall, Diameter of %0.2fm",
		height,
		2*math.Sqrt(volume/(math.Pi*height)),
	)
}

func calculateSphereFromVolume(volume float64) string {
	return fmt.Sprintf(
		"Sphere: %0.2fm Radius",
		math.Pow(3*volume/4/math.Pi, 1.0/3),
	)
}

func calculateConeFromVolume(volume float64) string {
	height := math.Pow(math.Pow(volume, 1.0/3), 2)
	return fmt.Sprintf(
		"Cone: %0.2fm tall, %0.2fm Radius",
		height,
		math.Sqrt(3*volume/math.Pi/height),
	)
}
