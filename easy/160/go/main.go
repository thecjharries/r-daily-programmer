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

type PromptTriangle struct {
	a, b, c, A, B, C float64
}

func NewPromptTriangle(partialValues PromptTriangle) (result *PromptTriangle) {
	result = new(PromptTriangle)
	result.C = 90
	if 0 < partialValues.a {
		result.a = partialValues.a
	}
	if 0 < partialValues.b {
		result.b = partialValues.b
	}
	if 0 < partialValues.c {
		result.c = partialValues.c
	}
	if 0 < partialValues.A {
		result.A = partialValues.A
	}
	if 0 < partialValues.B {
		result.B = partialValues.B
	}
	return
}

func (t *PromptTriangle) ComputeEdgeA() {
	if 0 < t.a {
		return
	}
	if 0 < t.c {
		if 0 < t.b {
			t.a = math.Sqrt(math.Pow(t.c, 2) - math.Pow(t.b, 2))
			return
		}
		if 0 < t.A {
			t.a = math.Sin(t.A) * t.c
			return
		}
		if 0 < t.B {
			t.a = math.Cos(t.B) * t.c
			return
		}
	}
	if 0 < t.A && 0 < t.b && 0 < t.B {
		t.a = math.Sin(t.A) * t.b / math.Sin(t.B)
		return
	}
}

func (t *PromptTriangle) ComputeEdgeB() {
	if 0 < t.b {
		return
	}
	if 0 < t.c {
		if 0 < t.a {
			t.b = math.Sqrt(math.Pow(t.c, 2) - math.Pow(t.a, 2))
			return
		}
		if 0 < t.B {
			t.b = math.Sin(t.B) * t.c
			return
		}
		if 0 < t.A {
			t.b = math.Cos(t.A) * t.c
			return
		}
	}
	if 0 < t.B && 0 < t.a && 0 < t.A {
		t.a = math.Sin(t.B) * t.a / math.Sin(t.A)
		return
	}
}

func (t *PromptTriangle) ComputeEdgeC() {
	if 0 < t.c {
		return
	}
	if 0 < t.a {
		if 0 < t.b {
			t.c = math.Sqrt(math.Pow(t.a, 2) + math.Pow(t.b, 2))
			return
		}
		if 0 < t.A {
			t.c = t.a / math.Sin(t.A)
			return
		}
		if 0 < t.B {
			t.c = t.a / math.Cos(t.B)
			return
		}
	} else if 0 < t.b {
		if 0 < t.A {
			t.c = t.b / math.Cos(t.A)
			return
		}
		if 0 < t.B {
			t.c = t.b / math.Sin(t.B)
			return
		}
	}
}

func (t *PromptTriangle) ComputeAngleA() {
	if 0 < t.A {
		return
	}
	if 0 < t.c {
		if 0 < t.a {
			t.A = math.Asin(t.a / t.c)
			return
		}
		if 0 < t.b {
			t.A = math.Acos(t.b / t.c)
			return
		}
	} else if 0 < t.B {
		t.A = math.Pi/2 - t.B
		return
	}
}

func (t *PromptTriangle) ComputeAngleB() {
	if 0 < t.B {
		return
	}
	if 0 < t.c {
		if 0 < t.a {
			t.B = math.Acos(t.a / t.c)
			return
		}
		if 0 < t.b {
			t.B = math.Asin(t.b / t.c)
			return
		}
	} else if 0 < t.A {
		t.B = math.Pi/2 - t.A
		return
	}
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
