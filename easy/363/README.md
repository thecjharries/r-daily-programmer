# [2018-06-11] Challenge #363 [Easy] I before E except after C

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/8q96da/20180611_challenge_363_easy_i_before_e_except/) by [u/Cosmologicon](https://old.reddit.com/user/Cosmologicon)

## Prompt

# Background

"I before E except after C" is perhaps the most famous English spelling rule. For the purpose of this challenge, the rule says:

* if "ei" appears in a word, it must immediately follow "c".
* If "ie" appears in a word, it must not immediately follow "c".

A word also follows the rule if neither "ei" nor "ie" appears anywhere in the word. Examples of words that follow this rule are:

    fiery hierarchy hieroglyphic
    ceiling inconceivable receipt
    daily programmer one two three

There are many exceptions that don't follow this rule, such as:

    sleigh stein fahrenheit
    deifies either nuclei reimburse
    ancient juicier societies

# Challenge

Write a function that tells you whether or not a given word follows the "I before E except after C" rule.

    check("a") => true
    check("zombie") => true
    check("transceiver") => true
    check("veil") => false
    check("icier") => false

# Optional Bonus 1

How many words in [the enable1 word list](https://norvig.com/ngrams/enable1.txt) are exceptions to the rule? (The answer is 4 digits long and the digits add up to 18.)

# Optional Bonus 2

This one is subjective and there's no best answer. Come up with your own "I before E" rule. Your rule must:

* depend on the ordering of the letters I and E when they appear next to each other. That is, if a word contains an I and an E next to each other, and it follows your rule, then when you swap those two letters, the new word must not follow your rule.
* depend only on the spelling of a word, not its pronunciation or meaning.
* be simple enough that schoolchildren can apply it.

For instance, I just came up with a rule "I before E, except when followed by G". This rule has 1,544 exceptions in the enable1 word list. How many exceptions does your rule have?
