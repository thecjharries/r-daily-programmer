# [2017-06-05] Challenge #318 [Easy] Countdown Game Show

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/6fe9cv/20170605_challenge_318_easy_countdown_game_show/) by [u/MoistedArnoldPalmer](https://old.reddit.com/u/MoistedArnoldPalmer) and [u/JakDrako](https://old.reddit.com/u/JakDrako)

## Prompt

# Description

This challenge is based off the British tv game show "Countdown". The rules are pretty simple: Given a set of numbers X1-X5, calculate using mathematical operations to solve for Y. You can use addition, subtraction, multiplication, or division.

Unlike "real math", the standard order of operations (PEMDAS) is not applied here. Instead, the order is determined left to right.

# Example Input

The user should input any 6 whole numbers and the target number. E.g.

    1 3 7 6 8 3 250

# Example Output

The output should be the order of numbers and operations that will compute the target number. E.g.

    3+8*7+6*3+1=250

Note that if follow PEMDAS you get:

	3+8*7+6*3+1 = 78

But remember our rule - go left to right and operate. So the solution is found by:

	(((((3+8)*7)+6)*3)+1) = 250

If you're into functional progamming, this is essentially a *fold* to the right using the varied operators.

# Challenge Input

	25 100 9 7 3 7 881

	6 75 3 25 50 100 952

# Challenge Output

	7 * 3 + 100 * 7 + 25 + 9 = 881

	100 + 6 * 3 * 75 - 50 / 25 = 952

# Notes about Countdown

Since Countdown's debut in 1982, there have been over 6,500 televised games and 75 complete series. There have also been fourteen Champion of Champions tournaments, with the most recent starting in January 2016.

On 5 September 2014, Countdown received a Guinness World Record at the end of its 6,000th show for the longest-running television programme of its kind during the course of its 71st series.

# Credit

This challenge was suggested by user /u/MoistedArnoldPalmer, many thanks. Furthermore, /u/JakDrako highlighted the difference in the order of operations that clarifies this problem significantly. Thanks to both of them. If you have a challenge idea, please share it in /r/dailyprogrammer_ideas and there's a good chance we'll use it.
