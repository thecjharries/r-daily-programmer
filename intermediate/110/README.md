# [11/3/2012] Challenge #110 [Intermediate] Creepy Crawlies

## Notes

Since reddit's decided to mess with the API, this isn't a viable prompt any more.

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/12k3xt/1132012_challenge_110_intermediate_creepy_crawlies/)

## Prompt

**Description:**

The web is full of creepy stories, with Reddit's /r/nosleep at the top of this list. Since you're a huge fan of not sleeping (we are programmers, after all), you need to amass a collection of creepy stories into a single file for easy reading access! Your goal is to write a web-crawler that downloads all the text submissions from the top 100 posts on /r/nosleep and puts it into a simple text-file.

**Formal Inputs & Outputs:**

*Input Description:*

No formal input: the application should simply launch and download the top 100 posts from /r/nosleep into a special file format.

*Output Description:*

Your application must either save to a file, or print to standard output, the following format: each story should start with a title line. This line is three equal-signs, the posts's name, and then three more equal-signs. An example is "=== People are Scary! ===". The following lines are the story itself, written in regular plain text. No need to worry about formatting, HTML links, bullet points, etc.

**Sample Inputs & Outputs:**

If I were to run the application now, the following would be examples of output:

=== Can I use the bathroom? ===

Since tonight's Halloween, I couldn't... (your program should print the rest of the story, I omit that for example brevity)

=== She's a keeper. ===

I love this girl with all of my... (your program should print the rest of the story, I omit that for example brevity)
