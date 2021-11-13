# [2017-08-01] Challenge #325 [Easy] Color maze

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/6qutez/20170801_challenge_325_easy_color_maze/) by [u/fvandepitte](https://old.reddit.com/user/fvandepitte)

## Prompt

#Description

Today we are going to do something colorfull and amazing. Yes it is a color maze :D (you can downvote me now, it was totally worth it).

You traverse a color by following a sequence of colors. For example [this maze](http://imgur.com/AnK1kwa) can be solved by the sequence 'orange -> green'.
Then you would have something like [this](http://imgur.com/zgciqa5) (paint skills)

For the mazes you always pick a spot on the bottom, in the starting color and try to get to the first row. Once you reach the first row, you are out of the maze. The sequence does not have to be complete.

You can move horizontally and vertically, but not diagonally. It is also allowed to move on the same node more then once.

#Formal Inputs & Outputs

##Input description

You will recieve a line with the sequence to follow and all the lines after that are the maze.

    O G
    B O R O Y
    O R B G R
    B O G O Y
    Y G B Y G
    R O R B R

##Output description

You can choose what you want to output:

you could output the path:

    (1,4)
    (1,3)
    (1,2)
    (2,2)
    (3,2)
    (3,1)
    (3,0)

or you could plot out the sequence

    / / / O /
    / / / G /
    / O G O /
    / G / / /
    / O / / /

or you could create an image result or go even fancier if you want to.


##Challnge Input

    R O Y P O
    R R B R R R B P Y G P B B B G P B P P R
    B G Y P R P Y Y O R Y P P Y Y R R R P P
    B P G R O P Y G R Y Y G P O R Y P B O O
    R B B O R P Y O O Y R P B R G R B G P G
    R P Y G G G P Y P Y O G B O R Y P B Y O
    O R B G B Y B P G R P Y R O G Y G Y R P
    B G O O O G B B R O Y Y Y Y P B Y Y G G
    P P G B O P Y G B R O G B G R O Y R B R
    Y Y P P R B Y B P O O G P Y R P P Y R Y
    P O O B B B G O Y G O P B G Y R R Y R B
    P P Y R B O O R O R Y B G B G O O P B Y
    B B R G Y G P Y G P R R P Y G O O Y R R
    O G R Y B P Y O P B R Y B G P G O O B P
    R Y G P G G O R Y O O G R G P P Y P B G
    P Y P R O O R O Y R P O R Y P Y B B Y R
    O Y P G R P R G P O B B R B O B Y Y B P
    B Y Y P O Y O Y O R B R G G Y G R G Y G
    Y B Y Y G B R R O B O P P O B O R R R P
    P O O O P Y G G Y P O G P O B G P R P B
    R B B R R R R B B B Y O B G P G G O O Y

#Notes/Hints

Since the sequence can have the same color more then once, it is possible that you have to visit the same node more then once.

#Bonus

Read the data not from text input but from the [image](http://imgur.com/uoItN6T)

All squares are 100 by 100 pixels with 1 pixel border

The RGB values are

    Red: (255, 0, 0)
    Green: (0,128,0)
    Blue: (0, 0, 255)
    Orange: (255, 165, 0)
    Yellow: (255, 255, 0)
    Pink: (255, 192, 203)

#Finally

Have a good challenge idea?

Consider submitting it to /r/dailyprogrammer_ideas

EDIT: Added clarifications after some questions of /u/the_droide
