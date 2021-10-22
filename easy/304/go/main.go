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
	"strconv"
	"strings"
)

type Account struct {
	Number  int
	Name    string
	Balance float64
}

func NewAccount(number int, args ...interface{}) Account {
	switch args[0].(type) {
	case string:
		return Account{
			Number: number,
			Name:   args[0].(string),
		}
	case float64:
		return Account{
			Number:  number,
			Balance: args[0].(float64),
		}
	}
	return Account{Number: number}
}

type Journal map[int]Account

func NewJournal(balanceInput, accountInput string) Journal {
	journal := make(Journal, 0)
	explodedBalances := strings.Split(balanceInput, "\n")
	for _, line := range explodedBalances {
		explodedLine := strings.Split(line, ";")
		number, _ := strconv.Atoi(explodedLine[0])
		debit, _ := strconv.ParseFloat(explodedLine[2], 64)
		credit, _ := strconv.ParseFloat(explodedLine[3], 64)
		account, exists := journal[number]
		if exists {
			account.Balance += debit - credit
		} else {
			journal[number] = NewAccount(number, debit-credit)
		}
	}
	explodedAccounts := strings.Split(accountInput, "\n")
	for _, line := range explodedAccounts {
		explodedLine := strings.Split(line, ";")
		number, _ := strconv.Atoi(explodedLine[0])
		account, exists := journal[number]
		if exists {
			account.Name = explodedLine[1]
		} else {
			journal[number] = NewAccount(number, explodedLine[1])
		}
	}
	return journal
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
