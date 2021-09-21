# [2016-05-23] Challenge #268 [Easy] Network and Cards: Part 1, The network

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/4knivr/20160523_challenge_268_easy_network_and_cards/) by [u/fvandepitte](https://old.reddit.com/user/fvandepitte)

## Prompt

#Description

This week we are creating a game playable over network. This will be a 3-parter.

The first part is to set up a connection between a server and one or more client.
The server needs to send out a heartbeat message to the clients and the clients need to respond to it.

For those who want to be prepared, we are going to deal and play cards over the network.

#Formal Inputs & Outputs

##Input description

No input for the server, but the client needs to know where the server is.

##Output description

The client needs to echo the heartbeat from the server.

#Notes/Hints

The server needs to able to handle multiple clients in the end, so a multithreaded approach is advised.
It is advised to think of some command like pattern, so you can send messages to the server and back.

For the server and client, just pick some random ports that you can use. [Here](https://en.wikipedia.org/wiki/List_of_TCP_and_UDP_port_numbers) you have a list off all "reserved" ports.

For the connection, TCP connections are the easiest way to do this in most languages. But you are not limited to that if you want to use something more high-level if your language of choice supports that.

#Bonus

- Make the server broadcast it's existince on the network, so clients can detect him.
- Send messages to the server and broadcast it to all the clients
- Let the client identify itself (username)
- Create a way to list all connected clients
- Send messages to the server and relay it to a requested client

These bonuses are not required, but it will make the next part a whole lot easier.

#Finally

Have a good challenge idea?

Consider submitting it to /r/dailyprogrammer_ideas
