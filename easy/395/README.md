# [2021-06-21] Challenge #395 [Easy] Nonogram row

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/o4uyzl/20210621_challenge_395_easy_nonogram_row/) by [u/oskar_s](https://old.reddit.com/u/oskar_s)

## Prompt

This challenge is inspired by [nonogram puzzles](https://en.wikipedia.org/wiki/Nonogram#Example), but you don't need to be familiar with these puzzles in order to complete the challenge.

A binary array is an array consisting of only the values `0` and `1`. Given a binary array of any length, return an array of positive integers that represent the lengths of the sets of consecutive 1's in the input array, in order from left to right.

    nonogramrow([]) => []
    nonogramrow([0,0,0,0,0]) => []
    nonogramrow([1,1,1,1,1]) => [5]
    nonogramrow([0,1,1,1,1,1,0,1,1,1,1]) => [5,4]
    nonogramrow([1,1,0,1,0,0,1,1,1,0,0]) => [2,1,3]
    nonogramrow([0,0,0,0,1,1,0,0,1,0,1,1,1]) => [2,1,3]
    nonogramrow([1,0,1,0,1,0,1,0,1,0,1,0,1,0,1]) => [1,1,1,1,1,1,1,1]

As a special case, nonogram puzzles usually represent the empty output (`[]`) as `[0]`. If you prefer to do it this way, that's fine, but `0` should not appear in the output in any other case.

*(This challenge is based on [Challenge #59 [intermediate]](https://www.reddit.com/r/dailyprogrammer/comments/uh03h/622012_challenge_59_intermediate/), originally posted by u/oskar_s in June 2012. Nonograms have been featured multiple times on r/dailyprogrammer since then ([search](https://www.reddit.com/r/dailyprogrammer/search?q=nonogram&restrict_sr=1)).)*
