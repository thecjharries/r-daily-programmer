# [2016-08-08] Challenge #278 [Easy/Med] Weave insert Part 1

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/4wqzph/20160808_challenge_278_easymed_weave_insert_part_1/) by [u/Godspiral](https://old.reddit.com/user/Godspiral)

## Prompt

This week's challenges are functional approaches to data interleaving.  This is usually done in the context of data being code or other machine (xml) representation.  Part 2 bonuses will be posted later in the week.

# input of 2 arrays

First array (or scalar) argument gets interleaved into 2nd array.  :

     insWeave([11]. [0, 1, 2, 3])
0 , 11 , 1 , 11 , 2 , 11 , 3

If first array is shorter than 2nd, it is extended cyclically

     insWeave([11,12]. [0, 1, 2, 3])
0 , 11 , 1  , 12 , 2 , 11 , 3

If the 2nd array is shorter than the first then the simplest option is to cut off the first array so that the 2 arrays have equal length.

     insWeave([11,12,13]. [0, 1])
0 , 11 , 1

# option 2:  shorter 2nd array is grouped by 2 and has items inserted in each pair. (strings are arrays of char)

     insBracket ('abc'  , '()' )
(a)
(b)
(c)


# string input

input format has each string within an array on its own line.  A blank line separates the 2 arrays.  A single string represents a character array.  The first line of input indicates "Bracket" or "Weave" to indicate use of the 2 alternate functions.

**input:**

Bracket
+-

234567

**output:**
2+3
4-5
6+7

**input:**
Bracket
2+3
4-5
6+7

()


**output:**
(2+3)
(4-5)
(6+7)

**input:**
Weave
*

(2+3)
(4-5)
(6+7)

**output:**
(2+3)
*
(4-5)
*
(6+7)

