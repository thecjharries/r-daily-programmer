# [2016-07-11] Challenge #275 [Easy] Splurthian Chemistry 101

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/4savyr/20160711_challenge_275_easy_splurthian_chemistry/) by [u/Cosmologicon](https://old.reddit.com/user/Cosmologicon)

## Prompt

# Description

The inhabitants of the planet Splurth are building their own [periodic table of the elements](https://en.wikipedia.org/wiki/Periodic_table#Overview). Just like Earth's periodic table has a chemical symbol for each element (`H` for Hydrogen, `Li` for Lithium, etc.), so does Splurth's. However, their chemical symbols must follow certain rules:

1. All chemical symbols must be exactly two letters, so `B` is not a valid symbol for Boron.
2. Both letters in the symbol must appear in the element name, but the first letter of the element name does not necessarily need to appear in the symbol. So `Hg` is not valid for Mercury, but `Cy` is.
3. The two letters must appear in order in the element name. So `Vr` is valid for Silver, but `Rv` is not. To be clear, both `Ma` and `Am` are valid for Magnesium, because there is both an `a` that appears after an `m`, and an `m` that appears after an `a`.
4. If the two letters in the symbol are the same, it must appear twice in the element name. So `Nn` is valid for Xenon, but `Xx` and `Oo` are not.

As a member of the Splurth Council of Atoms and Atom-Related Paraphernalia, you must determine whether a proposed chemical symbol fits these rules.

# Details

Write a function that, given two strings, one an element name and one a proposed symbol for that element, determines whether the symbol follows the rules. If you like, you may parse the program's input and output the result, but this is not necessary.

The symbol will have exactly two letters. Both element name and symbol will contain only the letters a-z. Both the element name and the symbol will have their first letter capitalized, with the rest lowercase. (If you find that too challenging, it's okay to instead assume that both will be completely lowercase.)

# Examples

    Spenglerium, Ee -> true
    Zeddemorium, Zr -> true
    Venkmine, Kn -> true
    Stantzon, Zt -> false
    Melintzum, Nn -> false
    Tullium, Ty -> false

# Optional bonus challenges

1. Given an element name, find the valid symbol for that name that's first in alphabetical order. E.g. `Gozerium -> Ei`, `Slimyrine -> Ie`.
2. Given an element name, find the number of distinct valid symbols for that name. E.g. `Zuulon -> 11`.
3. The planet Blurth has similar symbol rules to Splurth, but symbols can be any length, from 1 character to the entire length of the element name. Valid Blurthian symbols for `Zuulon` include `N`, `Uuo`, and `Zuuln`. Complete challenge #2 for the rules of Blurth. E.g. `Zuulon -> 47`.
