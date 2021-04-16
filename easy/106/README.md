# [10/23/2012] Challenge #106 [Easy] (Random Talker, Part 1)

## Note

The `determineTopTenWords` are not deterministic (AFAIK). Because the map is not ordered, the insertions into the final slice seem to be random per count. I don't want to solve this problem with an extra sort (I'm lazy), so the tests will occasionally fail.

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/11xje0/10232012_challenge_106_easy_random_talker_part_1/) by [u/spacemoses](https://old.reddit.com/user/spacemoses)

## Prompt

Your program will be responsible for analyzing a small chunk of text, the text of this ***entire*** dailyprogrammer description.  Your task is to output the distinct words in this description, from highest to lowest count with the number of occurrences for each.  Punctuation will be considered as separate words where they are not a part of the word.

The following will be considered their own words: **"** **.** **,** **:** **;** **!** **?** **(** **)** **[** **]** **{** **}**

For anything else, consider it as part of a word.

Extra Credit:

Process the text of the ebook [Metamorphosis, by Franz Kafka](http://www.gutenberg.org/cache/epub/5200/pg5200.txt) and determine the top 10 most frequently used words and their counts. (This will help for part 2)
