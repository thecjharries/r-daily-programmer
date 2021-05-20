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
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct{}

var _ = Suite(&MainSuite{})

var printCallCount int
var printSpyContents string

func printSpy(a ...interface{}) (n int, err error) {
	printSpyContents = fmt.Sprint(a...)
	printCallCount++
	return
}

func (s *MainSuite) SetUpTest(c *C) {
	printCallCount = 0
	printSpyContents = ""
	zPrint = printSpy
}

func (s *MainSuite) TearDownTest(c *C) {
	zPrint = fmt.Println
}

func (s *MainSuite) TestMain(c *C) {
	c.Assert(printCallCount, Equals, 0)
	c.Assert(printSpyContents, Equals, "")
	main()
	c.Assert(printCallCount, Equals, 1)
	c.Assert(printSpyContents, Equals, "hello world")
}

func (s *MainSuite) TestBuildingProcessRoomLogEntry(c *C) {
	rooms := make(map[int]*RoomData)
	building := Building{Rooms: rooms}
	c.Assert(len(building.Rooms) == 0, Equals, true)
	building.ProcessRoomLogEntry(RoomLogEntry{0, 0, true, 540})
	c.Assert(len(building.Rooms) == 1, Equals, true)
	building.ProcessRoomLogEntry(RoomLogEntry{0, 0, false, 560})
	c.Assert(len(building.Rooms) == 1, Equals, true)
	c.Assert(building.Rooms[0].VisitorCount, Equals, 1)
	c.Assert(building.Rooms[0].TotalTime, Equals, 20)
}

func (s *MainSuite) TestNewBuilding(c *C) {
	entries := []RoomLogEntry{
		{0, 0, true, 540},
		{0, 0, false, 560},
	}
	building := NewBuilding(entries)
	c.Assert(building.Rooms[0].VisitorCount, Equals, 1)
	c.Assert(building.Rooms[0].TotalTime, Equals, 20)
}

func (s *MainSuite) TestRoomDataString(c *C) {
	room := RoomData{
		TotalTime:    40,
		VisitorCount: 2,
	}
	c.Assert(room.String(0), Equals, "Room 0, 20 minute average visit, 2 visitor(s) total")
}

func (s *MainSuite) TestBuildingString(c *C) {
	var entries []RoomLogEntry
	var building Building
	entries = []RoomLogEntry{
		{0, 0, true, 540},
		{0, 0, false, 560},
	}
	building = NewBuilding(entries)
	c.Assert(building.String(), Equals, "Room 0, 20 minute average visit, 1 visitor(s) total\n")
	//	entries = []RoomLogEntry{
	//		{0, 11, true, 347},
	//		{1, 13, true, 307},
	//		{2, 15, true, 334},
	//		{3, 6, true, 334},
	//		{4, 9, true, 334},
	//		{5, 2, true, 334},
	//		{6, 2, true, 334},
	//		{7, 11, true, 334},
	//		{8, 1, true, 334},
	//		{0, 11, false, 376},
	//		{1, 13, false, 321},
	//		{2, 15, false, 389},
	//		{3, 6, false, 412},
	//		{4, 9, false, 418},
	//		{5, 2, false, 414},
	//		{6, 2, false, 349},
	//		{7, 11, false, 418},
	//		{8, 1, false, 418},
	//		{0, 12, true, 437},
	//		{1, 28, true, 343},
	//		{2, 32, true, 408},
	//		{3, 15, true, 458},
	//		{4, 18, true, 424},
	//		{5, 26, true, 442},
	//		{6, 7, true, 435},
	//		{7, 19, true, 456},
	//		{8, 19, true, 450},
	//		{0, 12, false, 455},
	//		{1, 28, false, 374},
	//		{2, 32, false, 495},
	//		{3, 15, false, 462},
	//		{4, 18, false, 500},
	//		{5, 26, false, 479},
	//		{6, 7, false, 493},
	//		{7, 19, false, 471},
	//		{8, 19, false, 458},
	//	}
	//	building = NewBuilding(entries)
	//	output := `Room 1, 85 minute average visit, 1 visitor total
	//Room 2, 48 minute average visit, 2 visitors total
	//Room 6, 79 minute average visit, 1 visitor total
	//Room 7, 59 minute average visit, 1 visitor total
	//Room 9, 85 minute average visit, 1 visitor total
	//Room 11, 57 minute average visit, 2 visitors total
	//Room 12, 19 minute average visit, 1 visitor total
	//Room 13, 15 minute average visit, 1 visitor total
	//Room 15, 30 minute average visit, 2 visitors total
	//Room 18, 77 minute average visit, 1 visitor total
	//Room 19, 12 minute average visit, 2 visitors total
	//Room 28, 32 minute average visit, 1 visitor total
	//Room 32, 88 minute average visit, 1 visitor total
	//`
	//	c.Assert(building.String(), Equals, output)
}
