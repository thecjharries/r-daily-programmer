# [6/22/2012] Challenge #68 [intermediate]

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/vfyj6/6222012_challenge_68_intermediate/)

## Prompt

You are given numbers N and H. H=floors of the building N=number of telephones. Your must find the MINIMUM amount of throws you need(A) to find out the highest floor from which the telephone will not break when thrown. For example when you have one phone and 10 floors, you start from the lowest floor and start throwing your phone- if it breaks the highest floor is 1, if not we throw it from the second floor-if it breaks the highest floor is 2....and so on. The problem asks you to find out the MINIMUM amount of throws it will take to find that floor. If N=1, then the answer is always H-because you start from the bottom and go up throwing the phone from every floor till it breaks.

Now when N>1 then that's a whole different story. If you have 2 phones you can throw one from the middle of the building, if it breaks, you only need to check floors 1-(middle-1) with your remaining phone, if it doesn't break you need to check floors (middle+1)-top floor.

* thanks to rollie82 for the challenge at /r/dailyprogrammer_ideas ! .. if you think you have a challenge worthy for our subreddit, go ahead and post it there!
