# [8/1/2012] Challenge #84 [easy] (Searching Text Adventure)

## Notes

This is one of the frustratingly vague prompts that isn't a good problem to solve. IIRC it's also a repeat of a lower number medium or hard problem.

My changed problem will be to get directions from the user and print grid location.

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/xilfu/812012_challenge_84_easy_searching_text_adventure/) by [u/Steve132](https://old.reddit.com/user/Steve132)

## Prompt

Like many people who program, I got started doing this because I wanted to learn how to make video games.

As a result, my first ever 'project' was also my first video game.  It involved a simple text adventure I called "The adventure of the barren moor"

In "The adventure of the barren moor" the player is in the middle of an infinite grey swamp.  This grey swamp has few distinguishing characteristics, other
than the fact that it is large and infinite and dreary.  However, the player DOES have a magic compass that tells the player how far away the next feature of interest is.

The player can go north,south,east,or west.  In my original version of the game, there was only one feature of interest, a treasure chest at a random point in the world.

Here is an example playthrough of my old program:

    You awaken to find yourself in a barren moor.  Try "look"
	> look
	Grey foggy clouds float oppressively close to you,
	reflected in the murky grey water which reaches up your shins.
	Some black plants barely poke out of the shallow water.
	Try "north","south","east",or "west"
	You notice a small watch-like device in your left hand.
	It has hands like a watch, but the hands don't seem to tell time.

	The dial reads '5m'

	>north
	The dial reads '4.472m'
	>north
	The dial reads '4.123m'
	>n
	The dial reads '4m'
	>n
	The dial reads '4.123m'
	>south
	The dial reads '4m'
	>e
	The dial reads '3m'
	>e
	The dial reads '2m'
	>e
	The dial reads '1m'
	>e

	You see a box sitting on the plain.   Its filled with treasure!  You win!  The end.

	The dial reads '0m'


Obviously, you do not have to use my flavor text, or my feature points.   As a matter of fact, its probably more interesting if you don't!
