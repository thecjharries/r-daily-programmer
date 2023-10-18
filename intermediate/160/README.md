# [4/30/2014] Challenge #160 Intermediate Part 2 - Damage Control

## Notes

The prompt isn't very well written. Looking at the solutions, it's possible to understand what the consensus is. However, there's not enough detail in the prompt itself to grok what's going on.

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/24da3f/4302014_challenge_160_intermediate_part_2_damage/)

## Prompt

[Part 1](http://www.reddit.com/r/dailyprogrammer/comments/236va2/4162014_challenge_158_intermediate_part_1_the/)

#Introduction

The new building techniques are a massive success, and soon it is adopted all across the far future society. However, suddenly a great swarm of high-tech termites are predicted to strike - and worse, due to a bug in /u/1337C0D3R's code, the design of the buildings are shoddy and are prone to being destroyed easily. If the buildings are destroyed by the army of termites this could lead to a crisis.

The slightly incompetent government of the future has realized that it is incumbent for them to act. They can provide you with a number of Reinforcement Kits 3000tm that when placed on a building, prevents the building from being destroyed. However, the Reinforcement Kit 3000tm is expensive to produce, so you decide to design an algorithm to use the least number of kits, and save the most money.
Description

The threatened buildings are placed in a straight line, numbered from 1 to N. Each building shares a wall with the buildings next to them - the adjacent buildings are known as 'neighbours'. This is an example of how the buildings would be set up for N = 12:

----------------------------------------------------
| 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 |
----------------------------------------------------

Each day the termites will start at one building and completely, irreversibly destroy it. After having destroyed the building, the termites will then spread to, but not destroy yet, all buildings that can be reached from the building that they started at. They cannot pass through buildings that are already destroyed. In other words, the termites cover all the area of a flood-fill from the starting building, with destroyed buildings as the boundary.

The termites will destroy the buildings that they have spread to unless a Reinforcement Kittm is placed on the building. After the termites have spread fully, you may begin placing kits. A Reinforcement Kittm will kill all termites in the building it is placed in. However, they only have an effect for one day; if on the next day the building again has termites another Reinforcement Kit must be used.

Given a list of P buildings that will be destroyed in P days, find the minimum number of Reinforcement Kits required, given that the buildings may be destroyed in any order. (The government has also given you Termite Bait which lets you choose the order in which the buildings in the list are destroyed).

#Formal Inputs and Outputs
Input Description

Input will be given on STDIN, or read from a file input.txt located in the working directory of the operating system. There will be exactly 2 lines of input. The first line contains two integers that are space separated, N and P. N is the number of buildings in the line. P is the number of buildings that will be destroyed in P days.

The second line consists of space-separated integers. The total number of integers will be equal to P. These are the indexes of the buildings which are to be destroyed.
Output Description

Output will be to STDOUT, or written to a file output.txt in the working directory. Output will contain a single integer consisting of the minimum number of Reinforcement Kits required.
#Sample Inputs and Outputs
#Sample Input 1

8 1

3

#Sample Output 1

7

#Sample Input 2

20 3

3 6 14

#Sample Output 2

35

#Notes

Thanks again to /u/202halffound
