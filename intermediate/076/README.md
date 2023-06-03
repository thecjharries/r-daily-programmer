# [7/13/2012] Challenge #76 [intermediate] (Probability graph)

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/wk066/7132012_challenge_76_intermediate_probability/)

## Prompt

Write a function `graph(f, low, high, tests)` that outputs a probability graph of the function `f` from range `low` to `high` (inclusive) over `tests` tests (i.e., counting the frequencies of `f()` outputs). `f` takes no arguments and returns an integer, `low`, `high` and `tests` are all integer values. For example, a function `f` that simulates two-dice rolls:

    def two_dice():
        return random.randint(1, 6) + random.randint(1, 6)

Then `graph(f, 2, 12, 10000)` should output something roughly like:

      2: ##
      3: #####
      4: #######
      5: ###########
      6: #############
      7: #################
      8: #############
      9: ###########
     10: ########
     11: #####
     12: ##

For bonus points, output the graph with the numbers on the bottom and the bars drawn vertically.
