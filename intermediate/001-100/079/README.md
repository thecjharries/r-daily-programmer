# [7/18/2012] Challenge #79 [intermediate] (Plain PGM file viewer)

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/wvcv9/7182012_challenge_79_intermediate_plain_pgm_file/)

## Prompt

Write a program that converts a ["plain" `.pgm` file](http://netpbm.sourceforge.net/doc/pgm.html) passed from stdin to an ASCII representation easily viewable in a terminal. If you're too lazy to read through the specification, the format should be simple enough to reverse-engineer from an example file:

    P2
    # feep.pgm
    24 7
    15
    0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0
    0  3  3  3  3  0  0  7  7  7  7  0  0 11 11 11 11  0  0 15 15 15 15  0
    0  3  0  0  0  0  0  7  0  0  0  0  0 11  0  0  0  0  0 15  0  0 15  0
    0  3  3  3  0  0  0  7  7  7  0  0  0 11 11 11  0  0  0 15 15 15 15  0
    0  3  0  0  0  0  0  7  0  0  0  0  0 11  0  0  0  0  0 15  0  0  0  0
    0  3  0  0  0  0  0  7  7  7  7  0  0 11 11 11 11  0  0 15  0  0  0  0
    0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0

* The top line, `P2`, is there to **identify** the file as a plain .pgm file.
* Lines with a **#** in front of them are **comments**, and should be ignored.
* The first two numbers in the file are the **width** and **height**.
* The third number, 15 here, is the **maximum grayscale value** in the image: here, this means 15 is full white, and lower numbers are darker, 0 being pure black.
* Thereafter, a **(*width* x *height*) grid** specifying the image itself follows.

Your program should use ASCII symbols to represent different grayscale values. Assuming the text is black on a white background, you could use a gradient like this one:

    " .:;+=%$#"

Converted, the example image would look something like this:


     ....  ;;;;  ====  ####
     .     ;     =     #  #
     ...   ;;;   ===   ####
     .     ;     =     #
     .     ;;;;  ====  #
