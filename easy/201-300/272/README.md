# [2016-06-20] Challenge #272 [Easy] What's in the bag?

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/4oylbo/20160620_challenge_272_easy_whats_in_the_bag/) by [u/genderdoom](https://old.reddit.com/u/genderdoom)

## Prompt

# Description

Scrabble is a popular word game where players remove tiles with letters on
them from a bag and use them to create words on a board. The total number
of tiles as well as the frequency of each letter does not change between
games.

For this challenge we will be using the tile set from the English edition,
which has 100 tiles total. [Here's a reference for the distribution and point
value of each tile](http://scrabblewizard.com/scrabble-tile-distribution/).

Each tile will be represented by the letter that appears on it, with the
exception that blank tiles are represented by underscores `_`.

# Input Description

The tiles already in play are inputted as an uppercase string. For example,
if 14 tiles have been removed from the bag and are in play, you would be given
an input like this:

    AEERTYOXMCNB_S

# Output Description

You should output the tiles that are left in the bag. The list should be in
descending order of the quantity of each tile left in the bag, skipping over
amounts that have no tiles.

In cases where more than one letter has the same quantity remaining, output
those letters in alphabetical order, with blank tiles at the end.

    10: E
    9: I
    8: A
    7: O
    5: N, R, T
    4: D, L, U
    3: G, S
    2: F, H, P, V, W
    1: B, C, J, K, M, Q, Y, Z, _
    0: X

If more tiles have been removed from the bag than possible, such as 3 `Q`s,
you should give a helpful error message instead of printing the list.

    Invalid input. More Q's have been taken from the bag than possible.

# Challenge Inputs

1. `PQAREIOURSTHGWIOAE_`

2. `LQTOONOEFFJZT`

3. `AXHDRUIOR_XHJZUQEE`

# Challenge Outputs

1.

10: E
7: A, I
6: N, O
5: T
4: D, L, R
3: S, U
2: B, C, F, G, M, V, Y
1: H, J, K, P, W, X, Z, _
0: Q

2.

11: E
9: A, I
6: R
5: N, O
4: D, S, T, U
3: G, L
2: B, C, H, M, P, V, W, Y, _
1: K, X
0: F, J, Q, Z

3.

Invalid input. More X's have been taken from the bag than possible.

# Bonus

After the normal output, output the distribution of tiles in play and the
total point score of both sets of tiles.

# Finally

Have a good challenge idea?
Consider submitting it to /r/dailyprogrammer_ideas

Thanks to /u/genderdoom for this [challenge idea](https://www.reddit.com/r/dailyprogrammer_ideas/comments/4j33t1/easy_whats_in_the_bag/).
