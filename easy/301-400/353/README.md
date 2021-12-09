# [2018-03-05] Challenge #353 [Easy] Closest String

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/826coe/20180305_challenge_353_easy_closest_string/) by [u/jnazario](https://old.reddit.com/user/jnazario)

## Prompt

# Description

In theoretical computer science, the closest string is an NP-hard computational problem, which tries to find the geometrical center of a set of input strings. To understand the word "center", it is necessary to define a distance between two strings. Usually, this problem is studied with the Hamming distance in mind. This center must be one of the input strings.

In bioinformatics, the closest string problem is an intensively studied facet of the problem of finding signals in DNA. In keeping with the bioinformatics utility, we'll use DNA sequences as examples.

Consider the following DNA sequences:

    ATCAATATCAA
    ATTAAATAACT
    AATCCTTAAAC
    CTACTTTCTTT
    TCCCATCCTTT
    ACTTCAATATA

Using the Hamming distance (the number of different characters between two sequences of the same length), the all-pairs distances of the above 6 sequences puts `ATTAAATAACT` at the center.

# Input Description

You'll be given input with the first line an integer *N* telling you how many lines to read for the input, then that number of lines of strings. All strings will be the same length. Example:

    4
    CTCCATCACAC
    AATATCTACAT
    ACATTCTCCAT
    CCTCCCCACTC

# Output Description

Your program should emit the string from the input that's closest to all of them. Example:

    AATATCTACAT

# Challenge Input

    11
    AACACCCTATA
    CTTCATCCACA
    TTTCAATTTTC
    ACAATCAAACC
    ATTCTACAACT
    ATTCCTTATTC
    ACTTCTCTATT
    TAAAACTCACC
    CTTTTCCCACC
    ACCTTTTCTCA
    TACCACTACTT

    21
    ACAAAATCCTATCAAAAACTACCATACCAAT
    ACTATACTTCTAATATCATTCATTACACTTT
    TTAACTCCCATTATATATTATTAATTTACCC
    CCAACATACTAAACTTATTTTTTAACTACCA
    TTCTAAACATTACTCCTACACCTACATACCT
    ATCATCAATTACCTAATAATTCCCAATTTAT
    TCCCTAATCATACCATTTTACACTCAAAAAC
    AATTCAAACTTTACACACCCCTCTCATCATC
    CTCCATCTTATCATATAATAAACCAAATTTA
    AAAAATCCATCATTTTTTAATTCCATTCCTT
    CCACTCCAAACACAAAATTATTACAATAACA
    ATATTTACTCACACAAACAATTACCATCACA
    TTCAAATACAAATCTCAAAATCACCTTATTT
    TCCTTTAACAACTTCCCTTATCTATCTATTC
    CATCCATCCCAAAACTCTCACACATAACAAC
    ATTACTTATACAAAATAACTACTCCCCAATA
    TATATTTTAACCACTTACCAAAATCTCTACT
    TCTTTTATATCCATAAATCCAACAACTCCTA
    CTCTCAAACATATATTTCTATAACTCTTATC
    ACAAATAATAAAACATCCATTTCATTCATAA
    CACCACCAAACCTTATAATCCCCAACCACAC

# Challenge Output

    ATTCTACAACT

    TTAACTCCCATTATATATTATTAATTTACCC

*EDITED* to correct the output of the first challenge.

# Bonus

Try this with various other [algorithms to measuring string similarity](https://en.wikipedia.org/wiki/String_metric), not just the Hamming distance.
