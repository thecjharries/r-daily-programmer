# [2018-01-08] Challenge #346 [Easy] Cryptarithmetic Solver

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/7p5p2o/20180108_challenge_346_easy_cryptarithmetic_solver/) by [u/fvandepitte](https://old.reddit.com/user/fvandepitte)

## Prompt

# Description

[Cryptarithms](https://en.wikipedia.org/wiki/Verbal_arithmetic) are a kind of mathematical puzzle. Each puzzle consists of a basic equation of arithmetic (involving addition, subtraction, division, etc.) with words, where each letter represents a different digit. The goal of the puzzle is to find the correct number substitution for each letter in order to make a valid equation.

This classic example (taken from the wikipedia page) was first published in 1924:


        S E N D
    +   M O R E
    _______________
      M O N E Y

The solution to this puzzle is:

O = 0,
M = 1,
Y = 2,
E = 5,
N = 6,
D = 7,
R = 8,
and S = 9.

(i.e. 9567 + 1085 = 10652)

Note: Leading zeroes are not allowed in a valid solution.

# Task

* You will be given a cryptarithm in string form. Your task is to output the letters and corresponding numbers which make up a valid solution to the puzzle.

* For the purposes of this challenge, all equations will consist only of addition.

* Leading zeroes (in a multi-digit number) are not allowed in a valid solution.

* The input is guaranteed to be a valid cryptarithm.


# Example

Input:
`"THIS + IS + HIS == CLAIM"`

Output:
`{"A"=>7, "C"=>1, "H"=>8, "I"=>5, "L"=>0, "M"=>6, "S"=>2, "T"=>9}`

# Challenge Input

    "WHAT + WAS + THY == CAUSE"

    "HIS + HORSE + IS == SLAIN"

    "HERE + SHE == COMES"

    "FOR + LACK + OF == TREAD"

    "I + WILL + PAY + THE == THEFT"

# Output

    {"A"=>0, "C"=>1, "E"=>4, "H"=>2, "S"=>3, "T"=>6, "U"=>7, "W"=>9, "Y"=>5}

    {"A"=>1, "E"=>8, "H"=>3, "I"=>5, "L"=>0, "N"=>6, "O"=>9, "R"=>7, "S"=>4}

    {"A"=>6, "C"=>7, "D"=>3, "E"=>2, "F"=>5, "K"=>8, "L"=>9, "O"=>4, "R"=>0, "T"=>1}

    {"A"=>2, "E"=>4, "F"=>7, "H"=>0, "I"=>8, "L"=>3, "P"=>5, "T"=>1, "W"=>9, "Y"=>6}

# Bonus

A bonus solution can solve one of the *longest known alphametics* in a reasonable amount of time:

    "TEN + HERONS + REST + NEAR + NORTH + SEA + SHORE + AS + TAN + TERNS + SOAR + TO + ENTER + THERE + AS + HERONS + NEST + ON + STONES + AT + SHORE + THREE + STARS + ARE + SEEN + TERN + SNORES + ARE + NEAR == SEVVOTH"

    "SO + MANY + MORE + MEN + SEEM + TO + SAY + THAT + THEY + MAY + SOON + TRY + TO + STAY + AT + HOME +  SO + AS + TO + SEE + OR + HEAR + THE + SAME + ONE + MAN + TRY + TO + MEET + THE + TEAM + ON + THE + MOON + AS + HE + HAS + AT + THE + OTHER + TEN == TESTS"

    "THIS + A + FIRE + THEREFORE + FOR + ALL + HISTORIES + I + TELL + A + TALE + THAT + FALSIFIES + ITS + TITLE + TIS + A + LIE + THE + TALE + OF + THE + LAST + FIRE + HORSES + LATE + AFTER + THE + FIRST + FATHERS + FORESEE + THE + HORRORS + THE + LAST + FREE + TROLL + TERRIFIES + THE + HORSES + OF + FIRE + THE + TROLL + RESTS + AT + THE + HOLE + OF + LOSSES + IT + IS + THERE + THAT + SHE + STORES + ROLES + OF + LEATHERS + AFTER + SHE + SATISFIES + HER + HATE + OFF + THOSE + FEARS + A + TASTE + RISES + AS + SHE + HEARS + THE + LEAST + FAR + HORSE + THOSE + FAST + HORSES + THAT + FIRST + HEAR + THE + TROLL + FLEE + OFF + TO + THE + FOREST + THE + HORSES + THAT + ALERTS + RAISE + THE + STARES + OF + THE + OTHERS + AS + THE + TROLL + ASSAILS + AT + THE + TOTAL + SHIFT + HER + TEETH + TEAR + HOOF + OFF + TORSO + AS + THE + LAST + HORSE + FORFEITS + ITS + LIFE + THE + FIRST + FATHERS + HEAR + OF + THE + HORRORS + THEIR + FEARS + THAT + THE + FIRES + FOR + THEIR + FEASTS + ARREST + AS + THE + FIRST + FATHERS + RESETTLE + THE + LAST + OF + THE + FIRE + HORSES + THE + LAST + TROLL + HARASSES + THE + FOREST + HEART + FREE + AT + LAST + OF + THE + LAST + TROLL + ALL + OFFER + THEIR + FIRE + HEAT + TO + THE + ASSISTERS + FAR + OFF + THE + TROLL + FASTS + ITS + LIFE + SHORTER + AS + STARS + RISE + THE + HORSES + REST + SAFE + AFTER + ALL + SHARE + HOT + FISH + AS + THEIR + AFFILIATES + TAILOR + A + ROOFS + FOR + THEIR + SAFE == FORTRESSES"

#Finally

Have a good challenge idea?

Consider submitting it to /r/dailyprogrammer_ideas

