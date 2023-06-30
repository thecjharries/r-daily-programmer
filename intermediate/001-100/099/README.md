# [9/17/2012] Challenge #99 [intermediate] (Unemployment map of the United States)

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/101mi5/9172012_challenge_99_intermediate_unemployment/)

## Prompt

[A little while ago](http://www.reddit.com/r/dailyprogrammer/comments/yj38u/8202012_challenge_89_difficult_coloring_the/) we took advantage of a [very useful blank map](http://en.wikipedia.org/wiki/File:Blank_US_Map.svg) hosted at Wikipedia. The advantage of this map is that it is very easy to assign different colors to each state (for details on how to do this, [see the previous problem](http://www.reddit.com/r/dailyprogrammer/comments/yj38u/8202012_challenge_89_difficult_coloring_the/)). We only had some silly fun with it, but it can also obviously be very useful in visualizing information about the country.

[Here is a text-file with unemployment data](https://gist.github.com/3740029) for all US states for each month from January 1980 to June 2012, stored in [CSV format](http://en.wikipedia.org/wiki/Comma-separated_values). The first column is the dates, then each column is the data for each state (the order of which is detailed in the headers). I got this information from the Federal Reserve Bank of St. Louis [FRED database](http://research.stlouisfed.org/fred2/), which has excellent API access (good work, St. Louis Fed!).

Using this table, make a program that can draw a map of unemployment across the United States at a given date. For instance, [here is a map of unemployment for July 2005](http://i.imgur.com/O4LP2.png). As you can see, I edited the map slightly, adding a scale to the left side and a header that includes the date. You can do that too if you wish, but it is not necessary in any way.

Your map doesn't need to look anything like mine. You can experiment with different colors and different styles. I selected the colors linearly based on unemployment, but you may want to use a different function to select colors, or perhaps color all states within a certain range the same (so that all states with 0%-2% are the same color, as are the states with 2%-4%, 4%-6%, etc). Experiment and see what you like.

Create a map which shows unemployment for February 1995.
