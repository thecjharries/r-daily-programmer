# [2017-09-04] Challenge #330 [Easy] Surround the circles

## Source

[Original post]() by [u/Preferencesoft](https://old.reddit.com/u/Preferencesoft)

## Prompt

# Description

In this challenge, you will be given a set of circles, defined by their centers and radii.  Your goal is to find the bounding rectangle which will contain all of the circles completely.

Write a program that determines the vertices of the bounding rectangle with sides parallel to the axes.

# Input Description

Each line will contain a comma separated center and radius for a circle.

# Output Description

The format of the output will be comma separated coordinates, rounded to 3 decimal places.

# Challenge Input

	1,1,2
    2,2,0.5
    -1,-3,2
    5,2,1

[input picture](https://i.imgur.com/uz6Bxqb.png)

# Challenge Output

	(-3.000, -5.000), (-3.000, 3.000), (6.000, 3.000), (6.000, -5.000)

[output picture](http://i.imgur.com/GAxlE8O.png)

# Bonus

For the bonus, we will rotate the axis for the bounding rectangle.  The first line of input will now be a vector determining the direction of one edge of the bounding rectangle.

# Bonus Input

	1,1
    1,1,2
    2,2,0.5
    -1,-3,2
    5,2,1

# Bonus Output

	(-4.828, -2.000), (2.793, 5.621), (6.621, 1.793), (-1.000, -5.828)

[bonus output picture](http://i.imgur.com/5IMZWPp.png)

# Credit

This challenge was suggested by user /u/Preferencesoft, many thanks! If you have an idea for a challenge please share it on /r/dailyprogrammer_ideas and there's a good chance we'll use it.
