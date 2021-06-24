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
	"image/color"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

// http://www.johndcook.com/blog/2009/08/24/algorithms-convert-color-grayscale/
func convertPixelToGreyscale(pixel color.Color) (newPixel color.Color) {
	red, green, blue, _ := pixel.RGBA()
	return color.Gray{uint8(0.21*float64(red) + 0.72*float64(green) + 0.07*float64(blue)/256)}
}
