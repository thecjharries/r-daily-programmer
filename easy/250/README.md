# [2016-01-18] Challenge #250 [Easy] Scraping /r/dailyprogrammer

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/41hp6u/20160118_challenge_250_easy_scraping/) by [u/fvandepitte](https://old.reddit.com/user/fvandepitte)

## Note

For consistency, I used [this link](https://api.reddit.com/r/dailyprogrammer.json?count=25&after=t3_onfehl) whose contents are in this dir. Also I didn't want to mess with the API right now because that's not an easy challenge.

Because I didn't feel like unmarshalling a bunch of stuff in Go, I ran this to create `simplified.json`:
```shell
cat dailyprogrammer.json | jq -r '[.data.children[].data]' > simplified.json
```

## Prompt

**Description**

As you all know, we have a not very wel updated [list of all the challenges](https://www.reddit.com/r/dailyprogrammer/wiki/challenges).

Today we are going to build a webscraper that creates that list for us, preferably using the [reddit api](https://www.reddit.com/dev/api).

Normally when I create a challenge I don't mind how you format input and output, but now, since it has to be [markdown](http://daringfireball.net/projects/markdown/syntax), I do care about the output.

---
Our List of challenges consist of a 4-column table, showing the Easy, Intermediate and Hard challenges, as wel as an extra's.


Easy | Intermediate | Hard | Weekly/Bonus
-----|--------------|------|-------------
| []() | []() | []() | **-** |
| [[2015-09-21] Challenge #233 [Easy] The house that ASCII built](/r/dailyprogrammer/comments/3ltee2/20150921_challenge_233_easy_the_house_that_ascii/) | []() | []() | **-** |
| [[2015-09-14] Challenge #232 [Easy] Palindromes](/r/dailyprogrammer/comments/3kx6oh/20150914_challenge_232_easy_palindromes/) | [[2015-09-16] Challenge #232 [Intermediate] Where Should Grandma's House Go?](/r/dailyprogrammer/comments/3l61vx/20150916_challenge_232_intermediate_where_should/) | [[2015-09-18] Challenge #232 [Hard] Redistricting Voting Blocks](/r/dailyprogrammer/comments/3lf3i2/20150918_challenge_232_hard_redistricting_voting/) | **-** |

The code code behind looks like this (minus the white line behind `Easy | Intermediate | Hard | Weekly/Bonus`):



    Easy | Intermediate | Hard | Weekly/Bonus

    -----|--------------|------|-------------
    | []() | []() | []() | **-** |
    | [[2015-09-21] Challenge #233 [Easy] The house that ASCII built](/r/dailyprogrammer/comments/3ltee2/20150921_challenge_233_easy_the_house_that_ascii/) | []() | []() | **-** |
    | [[2015-09-14] Challenge #232 [Easy] Palindromes](/r/dailyprogrammer/comments/3kx6oh/20150914_challenge_232_easy_palindromes/) | [[2015-09-16] Challenge #232 [Intermediate] Where Should Grandma's House Go?](/r/dailyprogrammer/comments/3l61vx/20150916_challenge_232_intermediate_where_should/) | [[2015-09-18] Challenge #232 [Hard] Redistricting Voting Blocks](/r/dailyprogrammer/comments/3lf3i2/20150918_challenge_232_hard_redistricting_voting/) | **-** |


**Input**

Not really, we need to be able to this.

**Output**

The entire table starting with the latest entries on top.
There won't be 3 challenges for each week, so take considuration. But challenges from the same week are with the same index number (e.g. `#1`, `#243`).

*Note*
We have changed the names from `Difficult` to `Hard` at some point

**Bonus 1**

It would also be nice if we could have the header generated. These are the 4 links you see at the top of /r/dailyprogrammer.

This is just a list and the source looks like this:

    1. [Challenge #242: **Easy**] (/r/dailyprogrammer/comments/3twuwf/20151123_challenge_242_easy_funny_plant/)
    2. [Challenge #242: **Intermediate**](/r/dailyprogrammer/comments/3u6o56/20151118_challenge_242_intermediate_vhs_recording/)
    3. [Challenge #242: **Hard**](/r/dailyprogrammer/comments/3ufwyf/20151127_challenge_242_hard_start_to_rummikub/)
    4. [Weekly #24: **Mini Challenges**](/r/dailyprogrammer/comments/3o4tpz/weekly_24_mini_challenges/)

**Bonus 2**

Here we do want to use an input.

We want to be able to generate just a one or a few rows by giving the rownumber(s)

*Input*

    213

*Output*

    | [[2015-09-07] Challenge #213 [Easy] Cellular Automata: Rule 90](/r/dailyprogrammer/comments/3jz8tt/20150907_challenge_213_easy_cellular_automata/) | [[2015-09-09] Challenge #231 [Intermediate] Set Game Solver](/r/dailyprogrammer/comments/3ke4l6/20150909_challenge_231_intermediate_set_game/) | [[2015-09-11] Challenge #231 [Hard] Eight Husbands for Eight Sisters](/r/dailyprogrammer/comments/3kj1v9/20150911_challenge_231_hard_eight_husbands_for/) | **-** |


*Input*

    229
    228
    227
    226

*Output*

    | [[2015-08-24] Challenge #229 [Easy] The Dottie Number](/r/dailyprogrammer/comments/3i99w8/20150824_challenge_229_easy_the_dottie_number/) | [[2015-08-26] Challenge #229 [Intermediate] Reverse Fizz Buzz](/r/dailyprogrammer/comments/3iimw3/20150826_challenge_229_intermediate_reverse_fizz/) | [[2015-08-28] Challenge #229 [Hard] Divisible by 7](/r/dailyprogrammer/comments/3irzsi/20150828_challenge_229_hard_divisible_by_7/) | **-** |
    | [[2015-08-17] Challenge #228 [Easy] Letters in Alphabetical Order](/r/dailyprogrammer/comments/3h9pde/20150817_challenge_228_easy_letters_in/) | [[2015-08-19] Challenge #228 [Intermediate] Use a Web Service to Find Bitcoin Prices](/r/dailyprogrammer/comments/3hj4o2/20150819_challenge_228_intermediate_use_a_web/) | [[08-21-2015] Challenge #228 [Hard] Golomb Rulers](/r/dailyprogrammer/comments/3hsgr0/08212015_challenge_228_hard_golomb_rulers/) | **-** |
    | [[2015-08-10] Challenge #227 [Easy] Square Spirals](/r/dailyprogrammer/comments/3ggli3/20150810_challenge_227_easy_square_spirals/) | [[2015-08-12] Challenge #227 [Intermediate] Contiguous chains](/r/dailyprogrammer/comments/3gpjn3/20150812_challenge_227_intermediate_contiguous/) | [[2015-08-14] Challenge #227 [Hard] Adjacency Matrix Generator](/r/dailyprogrammer/comments/3h0uki/20150814_challenge_227_hard_adjacency_matrix/) | **-** |
    | [[2015-08-03] Challenge #226 [Easy] Adding fractions](/r/dailyprogrammer/comments/3fmke1/20150803_challenge_226_easy_adding_fractions/) | [[2015-08-05] Challenge #226 [Intermediate] Connect Four](/r/dailyprogrammer/comments/3fva66/20150805_challenge_226_intermediate_connect_four/) | [[2015-08-07] Challenge #226 [Hard] Kakuro Solver](/r/dailyprogrammer/comments/3g2tby/20150807_challenge_226_hard_kakuro_solver/) | **-** |

**Note**
As /u/cheerse points out, you can use the [Reddit api wrappers](https://github.com/reddit/reddit/wiki/API-Wrappers) if available for your language
