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
	"math/rand"
	"time"
)

type Roll int

const (
	R00 = iota
	R0
	R1
	R2
	R3
	R4
	R5
	R6
	R7
	R8
	R9
	R10
	R11
	R12
	R13
	R14
	R15
	R16
	R17
	R18
	R19
	R20
	R21
	R22
	R23
	R24
	R25
	R26
	R27
	R28
	R29
	R30
	R31
	R32
	R33
	R34
	R35
	R36
)

var possibleRolls = [38]string{
	"00",
	"0",
	"1",
	"2",
	"3",
	"4",
	"5",
	"6",
	"7",
	"8",
	"9",
	"10",
	"11",
	"12",
	"13",
	"14",
	"15",
	"16",
	"17",
	"18",
	"19",
	"20",
	"21",
	"22",
	"23",
	"24",
	"25",
	"26",
	"27",
	"28",
	"29",
	"30",
	"31",
	"32",
	"33",
	"34",
	"35",
	"36",
}

var rollSpace = [38]Roll{
	R00,
	R0,
	R1,
	R2,
	R3,
	R4,
	R5,
	R6,
	R7,
	R8,
	R9,
	R10,
	R11,
	R12,
	R13,
	R14,
	R15,
	R16,
	R17,
	R18,
	R19,
	R20,
	R21,
	R22,
	R23,
	R24,
	R25,
	R26,
	R27,
	R28,
	R29,
	R30,
	R31,
	R32,
	R33,
	R34,
	R35,
	R36,
}

func (r *Roll) String() string {
	return possibleRolls[*r]
}

func NewRoll() Roll {
	return rollSpace[rand.Intn(R36 - R00) + R00]
}

type ValueModifier int

const (
	ValueModifierAny = iota
	ValueModifierAll
)

type Bet struct {
	name string
	values []Roll
	valueModifier ValueModifier
	payout string
}

var bets = []Bet{
	{"00", []Roll{R00}, ValueModifierAll, "35 to 1"},
	{"0", []Roll{R0}, ValueModifierAll, "35 to 1"},
	{"21", []Roll{R21}, ValueModifierAll, "35 to 1"},
	{"22", []Roll{R22}, ValueModifierAll, "35 to 1"},
	{"23", []Roll{R23}, ValueModifierAll, "35 to 1"},
	{"24", []Roll{R24}, ValueModifierAll, "35 to 1"},
	{"25", []Roll{R25}, ValueModifierAll, "35 to 1"},
	{"26", []Roll{R26}, ValueModifierAll, "35 to 1"},
	{"1st Column", []Roll{R1,R4,R7,R10,R13,R16,R19,R22,R25,R28,R31,R34}, ValueModifierAny, "2 to 1"},
}

var zSeed int64 = time.Now().UnixNano()

func main() {
	rand.Seed(0)
	bet := bets[5]
	fmt.Println(spin(bet))
}

func spin(bet Bet) string {
	payout := "none"
	roll := NewRoll()
	wonBet := false
	for _, possibleRoll := range bet.values {
		if ValueModifierAll == bet.valueModifier {
			if roll == possibleRoll {
				wonBet = true
			} else {
				wonBet = false
				break
			}
		}
		if roll == possibleRoll && ValueModifierAny == bet.valueModifier {
			wonBet = true
			break
		}
	}
	if wonBet {
		payout = bet.payout
	}
	return fmt.Sprintf("rolled %d: %s\n", roll, payout)
}
