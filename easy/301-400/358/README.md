# [2018-04-23] Challenge #358 [Easy] Decipher The Seven Segments

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/8eger3/20180423_challenge_358_easy_decipher_the_seven/) by [u/Garth5689](https://old.reddit.com/user/Garth5689)

## Prompt

# Description
Today's challenge will be to create a program to decipher a seven segment display, commonly seen on many older electronic devices.

# Input Description
For this challenge, you will receive 3 lines of input, with each line being 27 characters long (representing 9 total numbers), with the digits spread across the 3 lines.  Your job is to return the represented digits.  You don't need to account for odd spacing or missing segments.

# Output Description
Your program should print the numbers contained in the display.

# Challenge Inputs

        _  _     _  _  _  _  _
      | _| _||_||_ |_   ||_||_|
      ||_  _|  | _||_|  ||_| _|

        _  _  _  _  _  _  _  _
    |_| _| _||_|| ||_ |_| _||_
      | _| _||_||_| _||_||_  _|

     _  _  _  _  _  _  _  _  _
    |_  _||_ |_| _|  ||_ | ||_|
     _||_ |_||_| _|  ||_||_||_|

     _  _        _  _  _  _  _
    |_||_ |_|  || ||_ |_ |_| _|
     _| _|  |  ||_| _| _| _||_

# Challenge Outputs

    123456789
    433805825
    526837608
    954105592

# Ideas!

If you have an idea for a challenge please share it on /r/dailyprogrammer_ideas and there's a good chance we'll use it.
