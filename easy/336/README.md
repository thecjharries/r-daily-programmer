# [2017-10-16] Challenge #336 [Easy] Cannibal numbers

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/76qk58/20171016_challenge_336_easy_cannibal_numbers/) by [u/Lemvig42](https://old.reddit.com/u/Lemvig42)

## Prompt

# Description

Imagine a given set of numbers wherein some are cannibals. We define a cannibal as a larger number can eat a smaller number and **increase its value by 1**. There are no restrictions on how many numbers any given number can consume.   A number which has been consumed is **no longer available**.

Your task is to determine the number of numbers which can have a value equal to or greater than a specified value.

#Input Description

You'll be given two integers, _i_ and _j_, on the first line. _i_ indicates how many values you'll be given, and _j_ indicates the number of queries.

Example:

     7 2
     21 9 5 8 10 1 3
     10 15


Based on the above description, 7 is number of values that you will be given.  2 is the number of queries.

That means -
* Query 1 - How many numbers can have the value of at least 10
* Query 2 - How many numbers can have the value of at least 15

#Output Description

Your program should calculate and show the number of numbers which are equal to or greater than the desired number.  For the sample input given, this will be -

     4 2

##Explanation

For Query 1 -

The number 9 can consume the numbers 5 to raise its value to 10

The number 8 can consume the numbers 1 and 3 to raise its value to 10.

So including 21 and 10, we can get **four** numbers which have a value of at least 10.




For Query 2 -

The number 10 can consume the numbers 9,8,5,3, and 1 to raise its value to 15.

So including 21, we can get **two** numbers which have a value of at least 15.

# Credit

This challenge was suggested by user /u/Lemvig42, many thanks! If you have a challenge idea, please share it in /r/dailyprogrammer_ideas and there's a good chance we'll use it
