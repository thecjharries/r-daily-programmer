#[10/30/2012] Challenge #109 [Easy] Digits Check

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/12csk7/10302012_challenge_109_easy_digits_check/) by [u/nint22](https://old.reddit.com/user/nint22)

## Prompt

**Description:**

Write a function, where given a string, return true if it only contains the digits from 0 (zero) to 9 (nine). Else, return false.

**Formal Inputs & Outputs:**

*Input Description:*

string data - a given string that may or may not contains digits; will never be empty

*Output Description:*

Return True or False - true if the given string only contains digits, false otherwise

**Sample Inputs & Outputs:**

"123" should return  true. "123.123" should return a false. "abc" should return a false.

**Notes:**

This is a trivial programming exercise, but a real challenge would be to optimize this function for your language and/or environment. As a recommended reading, look into how [fast string-searching](http://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm) works.
