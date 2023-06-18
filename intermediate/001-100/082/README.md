# [7/27/2012] Challenge #82 [intermediate] (Broken Roman Numerals)

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/x8t42/7272012_challenge_82_intermediate_broken_roman/)

## Prompt

Many ancient buildings and scriptures use [Roman numerals](http://en.wikipedia.org/wiki/Roman_numerals) to express numbers or years. Let's say you discover a bunch of formulas using these numerals, but some of the letters used in them are unreadable, like this:

    LX?I

That ? could be an I, V, or X, giving you these possibilities:

    LXII = 62
    LXVI = 66
    LXXI = 71

Write a function `guess_roman` that outputs all possibilities for an input string containing any number of question marks. For example, `guess_roman("X??I")` outputs:

    XIII = 13
    XVII = 17
    XXII = 22
    XXVI = 26
    XXXI = 31
    XLII = 42
    XLVI = 46
    XCII = 92
    XCVI = 96

* What is the output of `guess_roman("D??I")`?
* How many unique seven-letter Roman numerals are there (that is, how many strings does `guess_roman("???????")` return?)
* What is their sum?
