# [9/30/2012] Challenge #102 [easy] (Dice roller)

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/10pf0j/9302012_challenge_102_easy_dice_roller/) by ???

## Prompt

In tabletop role-playing games like Dungeons & Dragons, people use a system called [dice notation](http://en.wikipedia.org/wiki/Dice_notation) to represent a combination of dice to be rolled to generate a random number. Dice rolls are of the form **A**d**B** *(+/-)* **C**, and are calculated like this:

1. Generate **A** random numbers from 1 to **B** and add them together.
2. Add or subtract the modifier, **C**.

If **A** is omitted, its value is 1; if (+/-)**C** is omitted, step 2 is skipped. That is, `"d8"` is equivalent to `"1d8+0"`.

Write a function that takes a string like `"10d6-2"` or `"d20+7"` and generates a random number using this syntax.

Here's a hint on how to parse the strings, if you get stuck:

    Split the string over 'd' first; if the left part is empty, A = 1,
    otherwise, read it as an integer and assign it to A. Then determine
    whether or not the second part contains a '+' or '-', etc.
