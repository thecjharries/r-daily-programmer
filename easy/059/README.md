# [6/2/2012] Challenge #59 [easy]

## Note

I've called my Go solution `findFirstIndexOfNaively` because a smarter solution exists. [The Knuth-Morris-Pratt algorithm](https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm) does a smarter search that tracks previously searched positions and uses them to set the next start point.

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/uh033/622012_challenge_59_easy/) by [u/oskar_s](https://old.reddit.com/user/oskar_s)

## Prompt

Write a program that given two strings, finds out if the second string is contained in the first, and if it is, where it is.

I.e. given the strings "Double, double, toil and trouble" and "il an" will return 18, because the second substring is embedded in the first, starting on position 18.

NOTE: Pretty much every language have this functionality built in for their strings, sometimes called find() (as in Python) or indexOf() (as in Java). But the point of this problem is to write the program yourself, so you **are not** allowed to use functions like this!
