# Challenge #270 [Easy] Transpose the input text

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/4msu2x/challenge_270_easy_transpose_the_input_text/) by [u/Gommie](https://old.reddit.com/u/Gommie)

## Prompt

#Description

Write a program that takes input text from standard input and outputs the text -- transposed.

Roughly explained, the transpose of a matrix

    A B C
    D E F

is given by

    A D
    B E
    C F


Rows become columns and columns become rows. See https://en.wikipedia.org/wiki/Transpose.

#Formal Inputs & Outputs

##Input description

One or more lines of text. Since the transpose is only valid for square matrices, append spaces to the shorter lines until they are of the same length. Characters may be multibyte (UTF-8) characters.

    Some
    text.

##Output description

The input text should be treated as a matrix of characters and flipped around the diagonal. I.e., the top right input character becomes the bottom left character of the output. Blank space at the end of output lines should be removed. Tab (\t) may be treated like any other character (don't replace it with spaces).

    St
    oe
    mx
    et
     .

Note that the lower left character is a space in the output, but nothing in the input.

## Input

    package main

    import "fmt"

    func main() {
        queue := make(chan string, 2)
        queue <- "one"
        queue <- "twoO"
        close(queue)
        for elem := range queue {
            fmt.Println(elem)
        }
    }

## Output

    p i f       }
    a m u
    c p n
    k o c
    a r  qqqcf }
    g t muuulo
    e   aeeeor
      " iuuus
    m f neeeeef
    a m (   (lm
    i t ):<<qet
    n "  =--um.
        {   e P
         m""u:r
         aote=i
         knw) n
         eeo rt
         ("O al
         c " nn
         h   g(
         a   ee
         n    l
             qe
         s   um
         t   e)
         r   u
         i   e
         n
         g   {
         ,

         2
         )

# Credit

This challenge was suggeted by /u/Gommie. Have a good challenge idea? Consider submitting it to /r/dailyprogrammer_ideas .
