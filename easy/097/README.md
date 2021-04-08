# [9/08/2012] Challenge #97 [easy] (Concatenate directory)

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/zkdv1/9082012_challenge_97_easy_concatenate_directory/) by ???

## Prompt

Write a program that concatenates all text files (`*.txt`) in a directory, numbering file names in alphabetical order. Print a header containing some basic information above each file.

For example, if you have a directory like this:

    ~/example/abc.txt
    ~/example/def.txt
    ~/example/fgh.txt

And call your program like this:

    nooodl:~$ ./challenge97easy example

The output would look something like this:

    === abc.txt (200 bytes)
    (contents of abc.txt)

    === def.txt (300 bytes)
    (contents of def.txt)

    === ghi.txt (400 bytes)
    (contents of ghi.txt)

For extra credit, add a command line option '`-r`' to your program that makes it recurse into subdirectories alphabetically, too, printing larger headers for each subdirectory.
