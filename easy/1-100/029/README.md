# [3/22/2012] Challenge #29 [easy]

## Note

The bonus is impossible. Because [the poem](https://users.math.yale.edu/public_html/People/frame/Fractals/Panorama/Literature/Martin/MartinPalindrome.html) contains punctuation that does not repeat correctly, the poem itself is not a palindrome. My assumption is that, because the poem is published with punctuation, the punctuation must be considered as part of the string.

The definition of palindrome from the prompt is explicitly `a sequence that is the same in reverse as it is forward`; the definition says nothing about only considering letters. If we used regex to only consider `/[a-z]/i` I suspect this would be a palindrome.

Also I'm too lazy to scrape it programmatically and I can't find licensing so I'm not going to include it here.

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/r8a70/3222012_challenge_29_easy/) by [u/_lerp](https://old.reddit.com/user/_lerp)

## Prompt

A [Palindrome](http://en.wikipedia.com/wiki/Palindrome) is a sequence that is the same in reverse as it is forward.

I.e. hannah, 12321.

Your task is to write a function to determine whether a given string is palindromic or not.


Bonus:  Support multiple lines in your function to validate Demetri Martin's [224 word palindrome poem](https://web.archive.org/web/20090413081928/http://www.pastemagazine.com/articles/2009/02/demetri-martins-palindrome-poem.html).


Thanks to _lerp for submitting this idea in /r/dailyprogrammer_ideas!
