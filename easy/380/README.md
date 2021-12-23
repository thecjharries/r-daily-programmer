# [2019-08-05] Challenge #380 [Easy] Smooshed Morse Code 1

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/cmd1hb/20190805_challenge_380_easy_smooshed_morse_code_1/) by [u/Separate_Memory](https://old.reddit.com/u/Separate_Memory)

## Prompt

For the purpose of this challenge, Morse code represents every letter as a sequence of 1-4 characters, each of which is either `.` (dot) or `-` (dash). The code for the letter `a` is `.-`, for `b` is `-...`, etc. The codes for each letter a through z are:

    .- -... -.-. -.. . ..-. --. .... .. .--- -.- .-.. -- -. --- .--. --.- .-. ... - ..- ...- .-- -..- -.-- --..

Normally, you would indicate where one letter ends and the next begins, for instance with a space between the letters' codes, but for this challenge, just smoosh all the coded letters together into a single string consisting of only dashes and dots.

# Examples

    smorse("sos") => "...---..."
    smorse("daily") => "-...-...-..-.--"
    smorse("programmer") => ".--..-.-----..-..-----..-."
    smorse("bits") => "-.....-..."
    smorse("three") => "-.....-..."

An obvious problem with this system is that decoding is ambiguous. For instance, both `bits` and `three` encode to the same string, so you can't tell which one you would decode to without more information.

# Optional bonus challenges

For these challenges, use the [enable1 word list](https://raw.githubusercontent.com/dolph/dictionary/master/enable1.txt). It contains 172,823 words. If you encode them all, you would get a total of 2,499,157 dots and 1,565,081 dashes.

1. The sequence `-...-....-.--.` is the code for four different words (`needing`, `nervate`, `niding`, `tiling`). Find the only sequence that's the code for 13 different words.
1. `autotomous` encodes to `.-..--------------..-...`, which has 14 dashes in a row. Find the only word that has 15 dashes in a row.
1. Call a word *perfectly balanced* if its code has the same number of dots as dashes. `counterdemonstrations` is one of two 21-letter words that's perfectly balanced. Find the other one.
1. `protectorate` is 12 letters long and encodes to `.--..-.----.-.-.----.-..--.`, which is a palindrome (i.e. the string is the same when reversed). Find the only 13-letter word that encodes to a palindrome.
1. `--.---.---.--` is one of five 13-character sequences that does not appear in the encoding of any word. Find the other four.

*Thanks to u/Separate_Memory for inspiring this challenge on r/dailyprogrammer_ideas!*
