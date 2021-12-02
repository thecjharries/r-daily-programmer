# [2017-11-21] Challenge #341 [Easy] Repeating Numbers

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/7eh6k8/20171121_challenge_341_easy_repeating_numbers/) by [u/MasterAgent47](https://old.reddit.com/user/MasterAgent47)

## Prompt

#Description

Locate all repeating numbers in a given number of digits. The size of the number that gets repeated should be more than 1. You may either accept it as a series of digits or as a complete number. I shall explain this with examples:

11**3259**92**321**9824**321**2**3259**

We see that:

+ 321 gets repeated 2 times
+ 32 gets repeated 4 times
+ 21 gets repeated 2 times
+ 3259 gets repeated 2 times
+ 25 gets repeated 2 times
+ 59 gets repeated 2 times

Or maybe you could have no repeating numbers:

1234565943210

You must consider such a case:

**987**020**987**040**9898**

Notice that 987 repeated itself twice (987, 987) and 98 repeated itself four times (98, 98, **98**7 and **98**7).

Take a chunk "9999". Note that there are three 99s and two 999s.

**99**99 9**99**9 9**999**

**999**9 9**999**

#Input Description

Let the user enter 'n' number of digits or accept a whole number.

#Output Description

> RepeatingNumber1:x RepeatingNumber2:y

If no repeating digits exist, then display 0.

Where x and y are the number of times it gets repeated.

#Challenge Input/Output

Input | Output
:--|--:
82156821568221 | 8215682:2 821568:2 215682:2 82156:2 21568:2 15682:2 8215:2 2156:2 1568:2 5682:2 821:2 215:2 156:2 568:2 682:2 82:3 21:3 15:2 56:2 68:2
11111011110111011 | 11110111:2 1111011:2 1110111:2 111101:2 111011:3 110111:2 11110:2 11101:3 11011:3 10111:2 1111:3 1110:3 1101:3 1011:3 0111:2 111:6 110:3 101:3 011:3 11:10 10:3 01:3
98778912332145 | 0
124489903108444899 | 44899:2 4489:2 4899:2 448:2 489:2 899:2 44:3 48:2 89:2 99:2

# Note

Feel free to consider '0x' as a two digit number, or '0xy' as a three digit number. If you don't want to consider it like that, it's fine.

-----------------------------------------------------------------

If you have any challenges, please submit it to /r/dailyprogrammer_ideas!


Edit: Major corrections by /u/Quantum_Bogo, error pointed out by /u/tomekanco
