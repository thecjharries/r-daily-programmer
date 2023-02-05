# [3/5/2012] Challenge #18 [intermediate]

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/qivpg/352012_challenge_18_intermediate/)

## Prompt

Screen scraping involves interacting with the terminal display of a currently running program.  There are commercial screen scraping applications available for mainframe programs that provide a web interface on top of a dumb terminal program.

Write a program that will read the following from a text file to simulate the dumb terminal program.  Each line represents a prompt to the user (always ends with a colon).  Input constraints may be available for prompts.  If they are, then they will always be surrounded in parentheses.  The accepted input values will always be separated by a comma, and the value that is actually counted will be surrounded in square brackets.

Once you've parsed the text file, convert the data into an HTML form output file.  If the prompt did not have any input constraints, then the input type is just a text.  If the prompt contained input constraints and there are less than 5 options, then the input type are radio buttons.  If there are 5 or more possible input values, then the input type is a dropdown.

Example:

Input File
---------

Name:

Gender ([M]ale, [F]emale):

Position ([C]ashier, [D]eli Clerk, [M]anager, [P]roduce Clerk, [S]tock Person):





Output File (HTML)
------------------

<html>

<body>

<form>

Name:

<input type="text" name="name"/>

<br/>

Gender:

<input type="radio" name="gender" value="m"/> Male

<input type="radio" name="gender" value="f"/> Female

<br/>

Position:

<select name="position">

<option value="c">Cashier</option>

<option value="d">Deli Clerk</option>

<option value="m">Manager</option>

<option value="p">Produce Clerk</option>

<option value="s">Stock Person</option>

</select>

<br/>

<input type="submit" value="Submit"/>

</form>

</body>

</html>
