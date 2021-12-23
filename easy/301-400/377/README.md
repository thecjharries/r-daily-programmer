# [2019-04-08] Challenge #377 [Easy] Axis-aligned crate packing

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/bazy5j/20190408_challenge_377_easy_axisaligned_crate/) by [u/Cosmologicon](https://old.reddit.com/user/Cosmologicon)

## Prompt

# Description

You have a 2-dimensional rectangular crate of size X by Y, and a bunch of boxes, each of size x by y. The dimensions are all positive integers.

Given X, Y, x, and y, determine how many boxes can fit into a single crate if they have to be placed so that the x-axis of the boxes is aligned with the x-axis of the crate, and the y-axis of the boxes is aligned with the y-axis of the crate. That is, you can't rotate the boxes. The best you can do is to build a rectangle of boxes as large as possible in each dimension.

For instance, if the crate is size X = 25 by Y = 18, and the boxes are size x = 6 by y = 5, then the answer is 12. You can fit 4 boxes along the x-axis (because 6\*4 <= 25), and 3 boxes along the y-axis (because 5\*3 <= 18), so in total you can fit 4*3 = 12 boxes in a rectangle.

# Examples

    fit1(25, 18, 6, 5) => 12
    fit1(10, 10, 1, 1) => 100
    fit1(12, 34, 5, 6) => 10
    fit1(12345, 678910, 1112, 1314) => 5676
    fit1(5, 100, 6, 1) => 0

# Optional bonus `fit2`

You upgrade your packing robot with the latest in packing technology: turning stuff. You now have the option of rotating *all* boxes by 90 degrees, so that you can treat a set of 6-by-5 boxes as a set of 5-by-6 boxes. You do **not** have the option of rotating some of the boxes but not others.

    fit2(25, 18, 6, 5) => 15
    fit2(12, 34, 5, 6) => 12
    fit2(12345, 678910, 1112, 1314) => 5676
    fit2(5, 5, 3, 2) => 2
    fit2(5, 100, 6, 1) => 80
    fit2(5, 5, 6, 1) => 0

*Hint: is there an easy way to define `fit2` in terms of `fit1`?*

Note that this is not the maximum possible number of boxes you could get if you rotated them independently. For instance, if you're fitting 3-by-2 boxes into a 5-by-5 crate, it's possible to fit 4 by varying the orientations, but `fit2(5, 5, 3, 2)` is 2, not 4. Handling the general case is much more complicated, and beyond the scope of today's challenge.

# Optional bonus `fit3`

You upgrade your warehouse to the third dimension. You're now given six parameters, X, Y, Z, x, y, and z. That is, you're given the X, Y, and Z dimensions of the crate, and the x, y, and z dimensions of the boxes. There are now six different possible orientations of the boxes. Again, boxes cannot be rotated independently: they all have to have the same orientation.

    fit3(10, 10, 10, 1, 1, 1) => 1000
    fit3(12, 34, 56, 7, 8, 9) => 32
    fit3(123, 456, 789, 10, 11, 12) => 32604
    fit3(1234567, 89101112, 13141516, 171819, 202122, 232425)) => 174648

# Optional bonus `fitn`

You upgrade your warehouse to the Nth dimension. Now you take a list of N crate dimensions, and N box dimensions. Assume that the boxes may be rotated in any of N! orientations so that each axis of the crate aligns with a different axis of the boxes. Again, boxes cannot be rotated independently.

    fitn([3, 4], [1, 2]) => 6
    fitn([123, 456, 789], [10, 11, 12]) => 32604
    fitn([123, 456, 789, 1011, 1213, 1415], [16, 17, 18, 19, 20, 21]) => 1883443968

EDIT: if you want even more of a challenge, do this in fewer than O(N!) operations. There's no specific time goal, but my Python program finds the following solution for N = 20 in about 10 seconds:

    fitn([180598, 125683, 146932, 158296, 171997, 204683, 193694, 216231, 177673, 169317, 216456, 220003, 165939, 205613, 152779, 177216, 128838, 126894, 210076, 148407], [1984, 2122, 1760, 2059, 1278, 2017, 1443, 2223, 2169, 1502, 1274, 1740, 1740, 1768, 1295, 1916, 2249, 2036, 1886, 2010]) => 4281855455197643306306491981973422080000
