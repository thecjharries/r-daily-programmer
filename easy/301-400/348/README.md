# [2018-01-22] Challenge #348 [Easy] The rabbit problem

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/7s888w/20180122_challenge_348_easy_the_rabbit_problem/) by [u/gaby_de_wilde](https://old.reddit.com/u/gaby_de_wilde)

## Prompt

**Description**

Rabbits are known for their fast breeding, but how soon will they dominate the earth?

Starting with a small population of male and female rabbits we have to figure out how long it will take for them to outnumber humans 2:1.

Every month a fertile female will have 14 offspring (5 males and 9 females).

A female rabbit is fertile when it has reached the age of 4 months, they never stop being fertile.

Rabbits die at the age of 8 years (96 months).

**Input description**

You will be given a list of numbers as following:

    Male_rabbits Female_rabbits Rabbits_needed_alive

The initial rabbits will always be 2 months old and fertile females will always produce 14 offspring (5 male, 9 female)

Every month that passes things should be done in this order:

1. Fertile female reproduce	(so 7 year & 11 months old will reproduce)
2. rabbits age (except newborn) (and rabbits reaching 8 years will die, the 7 year & 11 months old will die)

fx:

    2 4 1000000000

**Output description**

You output how many months it took for world domination

**Example**

Looking just at the female population

    we start with 1 female with the given starting age of 2 months

    the index is their age (0-index is 0 months old)
    [ 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0] Month 0
    [ 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0] Month 1
    [ 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0] Month 2
    [ 9, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0] Month 3
    [ 9, 9, 0, 0, 0, 0, 1, 0, 0, 0, 0] Month 4
    [ 9, 9, 9, 0, 0, 0, 0, 1, 0, 0, 0] Month 5
    [ 9, 9, 9, 9, 0, 0, 0, 0, 1, 0, 0] Month 6
    [ 9, 9, 9, 9, 9, 0, 0, 0, 0, 1, 0] Month 7
    [90, 9, 9, 9, 9, 9, 0, 0, 0, 0, 1] Month 8

**For Inspiration**

[The rabbit problem](http://go-here.nl/the-rabbit-problem.html)

**Challenge input(s)**

    2 4 1000000000

.

    2 4 15000000000

**Challenge output(s)**

    32

.

    36

**Bonus**

Tell how many dead rabbits there are when they dominate earth

**Credit**

This challenge was suggested by user /u/gaby_de_wilde, many thanks. If you have an idea for a challenge, please share it in /r/dailyprogrammer_ideas and there's a good chance we'll use it.
