# [8/3/2012] Challenge #85 [intermediate] (3D cuboid projection)

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/xq2ao/832012_challenge_85_intermediate_3d_cuboid/)

## Prompt

Write a program that outputs simple 3D ASCII art for a cuboid in an [oblique](http://en.wikipedia.org/wiki/Oblique_projection) perspective, given a length, height, and depth, like this:

    $ python 3d.py 20 10 3
       :::::::::::::::::::/
      :::::::::::::::::::/+
     :::::::::::::::::::/++
    ####################+++
    ####################+++
    ####################+++
    ####################+++
    ####################+++
    ####################+++
    ####################+++
    ####################++
    ####################+
    ####################

(The characters used for the faces (here `#`, `:`, and `+`) are fully up to you, but make sure you don't forget the `/` on the top-right edge.)
