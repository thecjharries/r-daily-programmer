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

const (
	batteryVoltage float64 = 9
	batteryAmps    float64 = 1200
	ledSerialCount float64 = 5
	ledSerialAmps  float64 = 20
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func calculateMaxLightsGivenDesiredHours(desiredHours float64) float64 {
	return math.Floor(math.Floor(batteryAmps/desiredHours/ledSerialAmps*ledSerialCount)/ledSerialCount) * ledSerialCount
}
