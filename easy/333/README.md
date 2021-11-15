# [2017-09-26] Challenge #333 [Easy] Packet Assembler

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/72ivih/20170926_challenge_333_easy_packet_assembler/) by [u/fvandepitte](https://old.reddit.com/user/fvandepitte)

## Prompt

#Description

When a message is transmitted over the internet, it is split into multiple packets, each packet is transferred individually, and the packets are reassembled into the original message by the receiver. Because the internet exists in the real world, and because the real world can be messy, packets do not always arrive in the order in which they are sent. For today's challenge, your program must collect packets from stdin, assemble them in the correct order, and print the completed messages to stdout.

The point of reading from stdin is to simulate incoming packets. For the purposes of this challenge, assume there is a potentially unlimited number of packets. Your program should not depend on knowing how many packets there are in total. Simply sorting the input in its entirety would technically work, but defeats the purpose of this exercise.

#Input description

Each line of input represents a single packet. Each line will be formatted as `X Y Z some_text`, where X Y and Z are positive integer and some_text is an arbitrary string. X represents the message ID (ie which message this packet is a part of). Y represents the packet ID (ie the index of this packet in the message) (packets are zero-indexed, so the first packet in a message will have Y=0, the last packet in a message will have Y=Z-1). Z represents the total number of packets in the message.

It is guaranteed that there will be no duplicate packets or message IDs.

##Example input

    6220	1	10	Because he's the hero Gotham deserves,
    6220	9	10
    5181	5	7	in time, like tears in rain. Time to die.
    6220	3	10	So we'll hunt him.
    6220	5	10	Because he's not a hero.
    5181	6	7
    5181	2	7	shoulder of Orion. I watched C-beams
    5181	4	7	Gate. All those moments will be lost
    6220	6	10	He's a silent guardian.
    5181	3	7	glitter in the dark near the Tannhäuser
    6220	7	10	A watchful protector.
    5181	1	7	believe. Attack ships on fire off the
    6220	0	10	We have to chase him.
    5181	0	7	I've seen things you people wouldn't
    6220	4	10	Because he can take it.
    6220	2	10	but not the one it needs right now.
    6220	8	10	A Dark Knight.

#Output description

Output each completed message, one line per packet. Messages should be outputted in the order in which they are completed.

##Example output

    5181	0	7	I've seen things you people wouldn't
    5181	1	7	believe. Attack ships on fire off the
    5181	2	7	shoulder of Orion. I watched C-beams
    5181	3	7	glitter in the dark near the Tannhäuser
    5181	4	7	Gate. All those moments will be lost
    5181	5	7	in time, like tears in rain. Time to die.
    5181	6	7
    6220	0	10	We have to chase him.
    6220	1	10	Because he's the hero Gotham deserves,
    6220	2	10	but not the one it needs right now.
    6220	3	10	So we'll hunt him.
    6220	4	10	Because he can take it.
    6220	5	10	Because he's not a hero.
    6220	6	10	He's a silent guardian.
    6220	7	10	A watchful protector.
    6220	8	10	A Dark Knight.
    6220	9	10

#Challenge input

    7469	1	7	believe. Attack ships on fire off the
    9949	6	10	He's a silent guardian.
    2997	9	19	Force is a pathway to many abilities some
    6450	2	11	is a vestige of the vox populi, now vacant, vanished. However, this valorous
    6450	10	11
    6450	8	11	veers most verbose, so let me simply add that it's my very good honour to meet
    6450	5	11	and voracious violation of volition! The only verdict is vengeance; a vendetta
    9949	1	10	Because he's the hero Gotham deserves,
    6450	1	11	and villain by the vicissitudes of fate. This visage, no mere veneer of vanity,
    2997	13	19	he did. Unfortunately, he taught his
    9949	8	10	A Dark Knight.
    1938	4	17	by the iniquities of the selfish and the
    1938	0	17	You read the Bible, Brett? Well there's
    2997	0	19	Did you ever hear the tragedy of Darth
    2997	1	19	Plagueis the Wise? I thought not. It's not a
    1938	8	17	of darkness, for he is truly is brother's
    2997	14	19	apprentice everything he knew, then his
    6450	3	11	visitation of a bygone vexation stands vivified, and has vowed to vanquish these
    1938	12	17	who attempt to poison and destroy my
    6450	9	11	you and you may call me V.
    7469	2	7	shoulder of Orion. I watched C-beams
    2997	10	19	consider to be unnatural. He became so
    1938	1	17	this passage I got memorized, sorta fits
    2997	5	19	Force to influence the midichlorians to
    1938	6	17	in the name of charity and good will,
    7469	0	7	I've seen things you people wouldn't
    9949	4	10	Because he can take it.
    6450	7	11	vindicate the vigilant and the virtuous. Verily, this vichyssoise of verbiage
    9949	0	10	We have to chase him.
    9949	7	10	A watchful protector.
    2997	3	19	legend. Darth Plagueis was a Dark Lord of the
    6450	6	11	held as a votive, not in vain, for the value and veracity of such shall one day
    2997	8	19	cared about from dying. The dark side of the
    1938	10	17	And I will strike down upon thee with
    1938	11	17	great vengeance and furious anger those
    1938	7	17	shepherds the weak through the valley
    1938	2	17	this occasion. Ezekiel 25:17? "The path
    2997	18	19
    9949	9	10
    1938	14	17	the Lord when I lay my vengeance upon
    1938	15	17	thee."
    1938	9	17	keeper and the finder of lost children.
    1938	13	17	brothers. And you will know my name is
    9949	2	10	but not the one it needs right now.
    2997	16	19	he could have others from death, but not
    2997	7	19	dark side that he could even keep the once he
    1938	5	17	tyranny of evil men. Blessed is he who,
    2997	17	19	himself.
    2997	6	19	create life...He had such a knowledge of the
    2997	12	19	losing his power. Which eventually, of course,
    7469	4	7	Gate. All those moments will be lost
    2997	2	19	story the Jedi would tell you. It's a Sith
    1938	16	17
    2997	4	19	Sith so powerful and so wise, he could use the
    1938	3	17	of the righteous man is beset on all sides
    2997	11	19	powerful...The only thing he was afraid of was
    7469	6	7
    2997	15	19	apprentice killed him in his sleep. Ironic,
    7469	5	7	in time, like tears in rain. Time to die.
    9949	3	10	So we'll hunt him.
    7469	3	7	glitter in the dark near the Tannhäuser
    6450	4	11	venal and virulent vermin vanguarding vice and vouchsafing the violently vicious
    6450	0	11	Voilà! In view, a humble vaudevillian veteran, cast vicariously as both victim
    9949	5	10	Because he's not a hero.

#Finally

Have a good challenge idea?

Consider submitting it to /r/dailyprogrammer_ideas
