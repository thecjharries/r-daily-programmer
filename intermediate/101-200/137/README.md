# [11/28/13] Challenge #137 [Intermediate / Hard] Banquet Planning

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/1rnrs2/112813_challenge_137_intermediate_hard_banquet/)

## Prompt

# [](#IntermediateIcon) *(Intermediate)*: Banquet Planning

You and your friends are planning a big banquet, but need to figure out the order in which food will be served. Some food, like a turkey, have to be served after appetizers, but before desserts. Other foods are more simple, like a pecan pie, which can be eaten any time after the main meal. Given a list of foods and the order-relationships they have, print the banquet schedule. If a given food item cannot be placed in this schedule, write an error message for it.

# Formal Inputs & Outputs
## Input Description

On standard console input, you will be given two space-delimited integers, N and M. N is the number of food items, while M is the number of food-relationships. Food-items are unique single-word lower-case names with optional underscores (the '_' character), while food-relationships are two food items that are space delimited. All food-items will be listed first on their own lines, then all food-relationships will be listed on their own lines afterwards. A food-relationship is where the first item must be served before the second item.

Note that in the food-relationships list, some food-item names can use the [wildcard-character](http://en.wikipedia.org/wiki/Wildcard_character) '\*'. You must support this by expanding the rule to fulfill any combination of strings that fit the wildcard. For example, using the items from Sample Input 2, the rule "turkey\* \*_pie" expands to the following four rules:

    turkey almond_pie
    turkey_stuffing almond_pie
    turkey pecan_pie
    turkey_stuffing pecan_pie

A helpful way to think about the wildcard expansion is to use the phrase "any item A must be before any item B". An example would be the food-relationship "\*pie coffee", which can be read as "any pie must be before coffee".

Some orderings may be ambiguous: you might have two desserts before coffee, but the ordering of desserts may not be explicit. In such a case, group the items together.

## Output Description

Print the correct order of food-items with a preceding index, starting from 1. If there are ambiguous ordering for items, list them together on the same line as a comma-delimited array of food-items. Any items that do not have a relationship must be printed with a warning or error message.

# Sample Inputs & Outputs
## Sample Input 1

    3 3
    salad
    turkey
    dessert
    salad dessert
    turkey dessert
    salad turkey

## Sample Output 1

    1. salad
    2. turkey
    3. dessert

## Sample Input 2

    8 5
    turkey
    pecan_pie
    salad
    crab_cakes
    almond_pie
    rice
    coffee
    turkey_stuffing
    turkey_stuffing turkey
    turkey* *_pie
    *pie coffee
    salad turkey*
    crab_cakes salad

## Sample Output 2

    1. crab_cakes
    2. salad
    3. turkey_stuffing
    4. turkey
    5. almond_pie, pecan_pie
    6. coffee

    Warning: Rice does not have any ordering.

# Author's Note:

This challenge has some subtle ordering logic that might be hard to understand at first. Work through sample data 2 by hand to better understand the ordering rules before writing code. Make sure to expand all widecard rules as well.
