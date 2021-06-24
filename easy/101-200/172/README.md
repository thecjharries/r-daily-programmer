# [7/21/2014] Challenge #172 [Easy] ■■□□□▦■□

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/2ba3g3/7212014_challenge_172_easy/) by [u/professorlamp](https://old.reddit.com/user/professorlamp)

## Prompt

#Description


A portable bitmap is one of the oldest image formats around and grants access to very simple image creation and sharing. Today, you will be creating an image of this format.



A simple PBM program can be seen [here] (http://en.wikipedia.org/wiki/Netpbm_format)
(Note that we'll be creating the simplest version, a PBM, not PPM or PGM.)


But basically the program consists of the following:



* A 2byte string (usually 'P1') denoting the file format for that PBM


* 2 integers denoting the Width and Height of our image file respectively


* And finally, our pixel data - Whether a pixel is 1 - Black or 0 - White.


#Formal Inputs & Outputs



##Input description

On standard console input you should be prompted to enter a small piece of text ("programming", "proggit", "hello world" etc...)



##Output description

The output will be a .PBM file consiting of an image which contains the text you have entered



#Notes


/u/chunes has kindly mapped all  alpha characters to their 0 1 equivalents, saving you a lot of time.


https://gist.github.com/anonymous/0ce707518d9e581499f5

Here is a worthwhile tutorial on the PBM format and programming for it


http://blog.plover.com/prog/perl/lines.html


The .PBM (you may also see it called NetPBM) is not very well supported any more, this makes actually viewing the PBM difficult as not many programs support it.

Feel free to download software which would render your .PBM to the screen but for all intents and purposes, the format is more important than the output cosidering the difficulty of viewing the image.


#Finally


Have a good challenge idea?

Consider submitting it to /r/dailyprogrammer_ideas
