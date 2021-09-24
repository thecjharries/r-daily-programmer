# [2016-06-27] Challenge #273 [Easy] Getting a degree

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/4q35ip/20160627_challenge_273_easy_getting_a_degree/) by [u/G33kDude](https://old.reddit.com/user/G33kDude)

## Prompt

# Description

Welcome to DailyProgrammer University. Today you will be earning a degree
in converting degrees. This includes Fahrenheit, Celsius, Kelvin,
Degrees (angle), and Radians.

# Input Description

You will be given two lines of text as input. On the first line, you will
receive a number followed by two letters, the first representing the unit
that the number is currently in, the second representing the unit it needs
to be converted to.

Examples of valid units are:

* `d` for degrees of a circle
* `r` for radians

# Output Description

You must output the given input value, in the unit specified. It must be
followed by the unit letter. You may round to a whole number, or to a few
decimal places.

# Challenge Input

    3.1416rd
    90dr

# Challenge Output

    180d
    1.57r

# Bonus

Also support these units:

* `c` for Celsius
* `f` for Fahrenheit
* `k` for Kelvin

If the two units given are incompatible, give an error message as output.

# Bonus Input

    212fc
    70cf
    100cr
    315.15kc

# Bonus Output

    100c
    158f
    No candidate for conversion
    42c

# Notes

* [See here](https://en.wikipedia.org/wiki/Conversion_of_units_of_temperature)
  for a wikipedia page with temperature conversion formulas.
* [See here](http://www.teacherschoice.com.au/maths_library/angles/angles.htm)
  for a random web link about converting between degrees and radians.

# Finally

Have a good challenge idea?
Consider submitting it to /r/dailyprogrammer_ideas
