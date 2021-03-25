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

func main() {
	for precision := int64(1); precision < 15; precision++ {
		numerator, denominator := pi(precision)
		fmt.Printf("%d/%d\n", numerator, denominator)
	}
}

// https://old.reddit.com/r/dailyprogrammer/comments/qx025/3142012_challenge_24_intermediate/c41505x/
func continuedFraction(n, m int64) (numerator, denominator int64) {
	if n < m {
		nextNumerator, nextDenominator := continuedFraction(n+1, m)
		numerator = (2*n-1)*nextNumerator + int64(math.Pow(float64(n), 2))*nextDenominator
		return numerator, nextNumerator
	}
	return 1, 1
}

// https://old.reddit.com/r/dailyprogrammer/comments/qx025/3142012_challenge_24_intermediate/c41505x/
func pi(precision int64) (int64, int64) {
	numerator, denominator := continuedFraction(1, precision)
	return 4 * denominator, numerator
}
