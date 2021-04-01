# [8/24/2012] Challenge #91 [easy] (Sleep sort)

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/yqydh/8242012_challenge_91_easy_sleep_sort/) by ???

## Prompt

An anonymous user on world4ch's programming text board posted a thread in early 2011 in which he describes an ingenious *O(n)* sorting algorithm. This means it's, supposedly, more efficient than any sorting algorithm ever invented. Some bloggers picked up on it, and dubbed the algorithm [sleep sort](http://beust.com/weblog/2011/06/15/sleep-sort/):


    #!/bin/bash
    function f() {
        sleep "$1"
        echo "$1"
    }
    while [ -n "$1" ]
    do
        f "$1" &
        shift
    done
    wait

This program takes some command line arguments, like `./sleepsort.sh 3 1 4 1 5 9`, and starts a new [thread](http://en.wikipedia.org/wiki/Thread_(computing\)) for each number in the list, which first sleeps for *n* seconds, then prints *n*. After 1 second, both 1s are printed, then after 2 more seconds the 3 follows, etc. Because it only loops through the list of numbers once, the algorithm appears to run in linear time.

Your task is to **re-implement sleep sort** in a language of your choice (which might look trivial, but this challenge is all about learning how your language handles multithreading.)

**BONUS**\: at first glance, this algorithm appears to be *O(n)*. Can you prove this isn't true? (This bonus requires some knowledge of both algorithms and concurrency.)
