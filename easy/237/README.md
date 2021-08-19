# [2015-10-19] Challenge #237 [Easy] Broken Keyboard

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/3pcb3i/20151019_challenge_237_easy_broken_keyboard/) by [u/ThinkinWithSand](https://old.reddit.com/u/ThinkinWithSand)

## Prompt

# Description

Help! My keyboard is broken, only a few keys work any more. If I tell you what keys work, can you tell me what words I can write?

(You should use the trusty [enable1.txt](http://norvig.com/ngrams/enable1.txt) file, or `/usr/share/dict/words` to chose your valid English words from.)

# Input Description

You'll be given a line with a single integer on it, telling you how many lines to read. Then you'll be given that many lines, each line a list of letters representing the keys that work on my keyboard. Example:

    3
    abcd
    qwer
    hjklo

# Output Description

Your program should emit the longest valid English language word you can make for each keyboard configuration.

    abcd = bacaba
    qwer = ewerer
    hjklo = kolokolo

# Challenge Input

    4
    edcf
    bnik
    poil
    vybu

# Challenge Output

    edcf = deedeed
    bnik = bikini
    poil = pililloo
    vybu = bubby

# Credit

This challenge was inspired by /u/ThinkinWithSand, many thanks! If you have any ideas, please share them on /r/dailyprogrammer_ideas and there's a chance we'll use it.
