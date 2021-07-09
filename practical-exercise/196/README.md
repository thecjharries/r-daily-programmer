# [2015-01-05] Challenge #196 [Practical Exercise] Ready... set... set!

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/2rfae0/20150105_challenge_196_practical_exercise_ready/) by [u/]( https://old.reddit.com/user/Elite6809)

## Prompt

# [](#PEIcon) **(Practical Exercise)**: Ready... set... Set!

The [last practical exercise](/r/dailyprogrammer/comments/2nr6c4/20141129_challenge_190_practical_exercise_the/) was well-received so I'm going to make another one. This one is less complicated and, if you're still finding your feet with object-oriented programming, should be great practice for you. This should be doable in functional languages too.

The idea of a Set can be very math-y when you delve deeper but this post only skims the surface, so it shouldn't pose any issue!

## Background

A *set* is a mathematical concept that represents a collection of other objects. Those other objects can be numbers, words, operations or even sets themselves; for the (non-extension) purposes of the challenge they are integers only. A *finite set* is a set with only a finite number of items (unlike, for example, the set of all real numbers **R** which has uncountably infinite members.)

A set is generally represented with curly brackets with the items separated by commas. So, for example, the set containing `-3`, `6` and `11` could be written as `{-3, 6, 11}`. This notation is called an *extensional definition*.

There are some distinctions between a set and the list/array data structure:

* Repeated items are ignored, so `{-3, 6, 11}` is exactly the same as `{-3, -3, 6, 11}`. To understand why this is so, think less of a set being a container of items, but rather the items are members of a set - much like how you can't be a subscriber on /r/DailyProgrammer twice.

* Order doesn't matter - `{-3, 6, 11}` is the same as `{6, 11, -3}` and so on.

* Sets are generally seen as *immutable*, which means that rather than adding an item **A** to a set **S**, you normally create a new set with all the members of **S**, *and* **A**. Immutable data structures are quite a common concept so this will serve as an intro to them if you've not came across them already.

* A set can be empty - `{}`. This is called the empty set, weirdly enough.

Sets have 3 main operations.

* Union, with the symbol ∪. An item is a member of set **S**, where **S**=**A**∪**B**, if it's a member of set **A** or set **B**.
  For example, let **A**=`{1, 4, 7}` and let **B**=`{-4, 7, 10}`. Then, **A**∪**B**=`{-4, 1, 4, 7, 10}`.

* Intersection, with the symbol ∩. An item is a member of set **S**, where **S**=**A**∩**B**, if it is a member of set **A** *and* set **B**.
  For example, let **A**=`{1, 4, 7}` and let **B**=`{-4, 7, 10}`. Then, **A**∩**B**=`{7}`, as only 7 is a member of both sets.

* Complement, with the symbol '. An item is a member of set **S**, where **S**=**A**', if it's not a member of **A**.
  For example, let **A**=`{1, 4, 7}`. Then, **A**' is *every integer* except 1, 4 and 7.

# Specification

You are to implement a class representing a set of integers.

* To hold its content, you can use an array, list, sequence or whatever fits the language best. Consider encapsulating this (making it `private`) if your language supports it.

* The class should expose a method `Contains`, which accepts an integer and returns whether the set contains that integer or not.

* The constructor of the class should accept a list/array of integers which are to be the content of the set. Remember to ignore duplicates and order. Consider making it a variadic constructor (variable number of arguments) if your language supports it.

* The class should have static methods for `Union` and `Intersection`, which both accept two sets and return another set containing the union or intersection of those two sets respectively. Remember, our sets are *immutable*, so create a new set rather tham modifying an existing one. Consider making these as binary operators (eg. `+` for union and `*` for intersection) if your language supports it.

* The class should have another static method for `Equals` or `equals`, which accepts two sets and returns a boolean value. This determines if the two sets contain the same items and nothing else.

Finally, the set should be convertible to a string in some way (eg. `ToString`, `toString`, `to_s` depending on the language) which shows all items in the set. It should show them in increasing order for readability.

If your language already has a class for sets, ignore it. The purpose of this exercise is to learn from implementing the class, not use the pre-existing class (although in most cases you would use the existing one.)

## Making Use of your Language

The main challenge of this exercise is knowing your language and its features, and adapting your solution to them. For example, in Ruby, you would not write a `ToString` method - you would write a `to_s` method, as that is the standard in Ruby. In C++ and C#, you would not necessarily write static `Union`, `Intersection` methods - you have the ability to overload operators, and you should do so if it produces [idiomatic code](http://en.wikipedia.org/wiki/Programming_idiom). The research for this is part of the task. You should also be writing clean, legible code. Follow the style guide for your language, and use the correct naming/capitalisation conventions, which differ from language to language.

## Extension 1 (Intermediate)

If you are feeling up to it, change your class for a set of integers and create a [generic](http://en.wikipedia.org/wiki/Generic_programming) set class (or, if your language has dynamic typing, a set of any comparable type.) Depending on your language you might need to specify that the objects can be equated - I know in C# this is by `IEquatable` but other language will differ. Some languages like Ruby don't even need to.

## Extension 2 (Hard)

This will require some thinking on your end. Add a `Complement` static method to your class, which accepts a set and returns another set containing everything *except* what's in the accepted set.
Of course, you can't have an array of every integer ever. You'll need to use another method to solve this extension, and adapt the rest of the class accordingly. Similarly, for the string conversion, you can't print an infinite number of items. For this reason, a set containing everything containing everything except 3 and 5 should print something like `{3, 5}'` (note the `'`). You could similarly use an overloaded operator for this - I've picked `!` in my solution.

## Addendum

Happy new year! I know /u/Coder_d00d has already wished you so, but now I do too. Have fun doing the challenge, help each other out and good luck for the new year.
