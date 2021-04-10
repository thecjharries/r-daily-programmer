# [9/01/2012] Challenge #94 [easy] (Elemental symbols in strings)

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/z6o4k/9012012_challenge_94_easy_elemental_symbols_in/) by ???

## Prompt

If you've ever seen *Breaking Bad*, you might have noticed how some [names in the opening credit sequence](http://i.imgur.com/qnul0.jpg) get highlights according to symbols of elements in the [periodic table](http://en.wikipedia.org/wiki/Periodic_table). Given a string as input, output every possible such modification with the element symbol enclosed in brackets and capitalized. The elements can appear anywhere in the string, but you must only highlight one element per line, like this:

    $ ./highlight dailyprogrammer
    dailypr[O]grammer
    daily[P]rogrammer
    dail[Y]programmer
    da[I]lyprogrammer
    dailyprog[Ra]mmer
    daily[Pr]ogrammer
    dailyprogramm[Er]
    dailyprogr[Am]mer
