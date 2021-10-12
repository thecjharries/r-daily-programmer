# [2016-09-19] Challenge #284 [Easy] Wandering Fingers

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/53ijnb/20160919_challenge_284_easy_wandering_fingers/) by [u/fj2010](https://old.reddit.com/u/fj2010)

## Prompt


#Description

Software like Swype and SwiftKey lets smartphone users enter text by *dragging* their finger over the on-screen keyboard, rather than tapping on each letter.

[Example image of Swype](http://www.swype.com/content/uploads/2014/09/swype_path.png)

You'll be given a string of characters representing the letters the user has dragged their finger over.

For example, if the user wants "rest", the string of input characters might be "resdft" or "resert".

#Input

Given the following input strings, find all possible output words 5 characters or longer.

1. qwertyuytresdftyuioknn
2. gijakjthoijerjidsdfnokg


#Output

Your program should find all possible words (5+ characters) that can be derived from the strings supplied.

Use http://norvig.com/ngrams/enable1.txt as your search dictionary.

The order of the output words doesn't matter.

1. queen question
2. gaeing garring gathering gating geeing gieing going goring

#Notes/Hints

Assumptions about the input strings:

* QWERTY keyboard
* Lowercase a-z only, no whitespace or punctuation
* The first and last characters of the input string will always match the first and last characters of the desired output word
* Don't assume users take the most efficient path between letters
* Every letter of the output word will appear in the input string


#Bonus

Double letters in the output word *might* appear only once in the input string, e.g. "polkjuy" could yield "polly".

Make your program handle this possibility.

# Credit

This challenge was submitted by /u/fj2010, thank you for this! If you have any challenge ideas please share them in /r/dailyprogrammer_ideas and there's a chance we'll use them.
