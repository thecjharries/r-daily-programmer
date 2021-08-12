# [2015-07-27] Challenge #225 [Easy/Intermediate] De-columnizing

## Source

[Original post](https://old.reddit.com/r/dailyprogrammer/comments/3esrkm/20150727_challenge_225_easyintermediate/) by [u/Elite6809](https://old.reddit.com/user/Elite6809)

## Prompt

# [](#EasyIcon) _(Easy/Intermediate)_: De-columnizing

Often, column-style writing will put images and features to the left or right of the body of text, for example:

    24
    This is an example piece of text. This is an exam-
    ple piece of text. This is an example piece of
    text. This is an example
    piece of text. This is a +-----------------------+
    sample for a challenge.  |                       |
    Lorum ipsum dolor sit a- |       top class       |
    met and other words. The |        feature        |
    proper word for a layout |                       |
    like this would be type- +-----------------------+
    setting, or so I would
    imagine, but for now let's carry on calling it an
    example piece of text. Hold up - the end of the
                     paragraph is approaching - notice
    +--------------+ the double line break for a para-
    |              | graph.
    |              |
    |   feature    | And so begins the start of the
    |   bonanza    | second paragraph but as you can
    |              | see it's only marginally better
    |              | than the other one so you've not
    +--------------+ really gained much - sorry. I am
                     certainly not a budding author
    as you can see from this example input. Perhaps I
    need to work on my writing skills.

In order to fit into the column format, some words are hyphenated. For the purpose of the challenge, you may assume that any hyphens at the end of a line join a single un-hyphenated word together (for example, the `exam-` and `ple` in the above input form the word `example` and not `exam-ple`). However, hyphenated words that do not span multiple lines should retain their hyphens.  Side features will only appear at the far left or right of the input, and will always be bordered by the `+---+` style shown above. They will also never have 'holes' in them, like this:

    +--------------------+
    |                    |
    | Inside the feature |
    |                    |
    | +----------------+ |
    | |                | |
    | |     Outside    | |
    | |                | |
    | +----------------+ |
    |                    |
    +--------------------+

Paragraphs in the input are separated by double line breaks, like Reddit markdown. Your task today is to extract just the paragraph text from the input, removing the feature-boxes.

# Formal Inputs and Outputs

## Input Specification

You'll be given a number **N** on one line, followed by **N** further lines of input like the example in the description above.

## Output Description

Output just the paragraph text, de-hyphenating words where appropriate (ie. a line of text ends with a hyphen).

# Sample Inputs and Outputs

## Example 1

This corresponds to the input given in the Description.

### Output

This is an example piece of text. This is an example piece of text. This is an example piece of text. This is an example piece of text. This is a sample for a challenge. Lorum ipsum dolor sit amet and other words. The proper word for a layout like this would be typesetting, or so I would imagine, but for now let's carry on calling it an example piece of text. Hold up - the end of the paragraph is approaching - notice the double line break for a paragraph.

And so begins the start of the second paragraph but as you can see it's only marginally better than the other one so you've not really gained much - sorry. I am certainly not a budding author as you can see from this example input. Perhaps I need to work on my writing skills.

## Example 2

### Input

    22
    +-------------+ One hundred and fifty quadrillion,
    |             | seventy-two trillion, six hundred
    | 150 072 626 | and twenty-six billion, eight hun-
    | 840 312 999 | dred and fourty million, three
    |             | hundred and thirteen thousand sub-
    +-------------+ tract one is a rather large prime
                    number which equals one to five if
    calculated modulo two to six respectively.

    However, one other rather more in- +-------------+
    teresting number is two hundred    |             |
    and twenty-one quadrillion, eight  | 221 806 434 |
    hundred and six trillion, four     | 537 978 679 |
    hundred and thirty-four billion,   |             |
    five hundred and thirty-seven mil- +-------------+
    million, nine hundred and seven-
                                    ty-eight thousand,
    +-----------------------------+ six hundred and
    |                             | seventy nine,
    | Subscribe for more Useless  | which isn't prime
    |      Number Facts(tm)!      | but is the 83rd
    +-----------------------------+ Lucas number.


### Output

One hundred and fifty quadrillion, seventy-two trillion, six hundred and twenty-six billion, eight hundred and fourty million, three hundred and thirteen thousand subtract one is a rather large prime number which equals one to five if calculated modulo two to six respectively.

However, one other rather more interesting number is two hundred and twenty-one quadrillion, eight hundred and six trillion, four hundred and thirty-four billion, five hundred and thirty-seven milmillion, nine hundred and seventy-eight thousand, six hundred and seventy nine, which isn't prime but is the 83rd Lucas number.

## Example 3

### Input

    16
    +----------------+ Lorem ipsum dolor sit amet,
    |                | consectetur adipiscing elit,
    |  Aha, now you  | sed do eiusmod tempor incid-
    |  are stumped!! | idunt ut labore et dolore
    |                | magna aliqua. Ut enim ad mi-
    |       +--------+ nim veniam, quis nostrud ex-
    |  top  |          ercitation ullamco laboris
    |  kek  | nisi ut aliquip ex.
    |       |                       +-------------+
    +-------+ Duis aute irure dolor |             |
    in repre-henderit in voluptate  | Nothing to  |
    velit esse cillum dolore eu fu- |  see here.  |
    giat nulla pariatur. Excepteur  |             |
    sint occaecat cupidatat non     +-------------+
    proident, sunt in culpa qui of-
    ficia deserunt mollit anim id est laborum.

### Output

Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex.

Duis aute irure dolor in repre-henderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

## [](#IntermediateIcon) Extension (Intermediate)

At the start of each paragraph in your output, list the text of each feature associated with that paragraph. A feature is "associated" with a paragraph if the top of the feature box (the `+--------+`) starts on or below the line that the paragraph starts on. For example, the outputs for the above three examples would be:

### Example 1 Output

(top class feature) (feature bonanza) This is an example piece of text. This is an example piece of text. This is an example piece of text. This is an example piece of text. This is a sample for a challenge. Lorum ipsum dolor sit amet and other words. The proper word for a layout like this would be typesetting, or so I would imagine, but for now let's carry on calling it an example piece of text. Hold up - the end of the paragraph is approaching - notice the double line break for a paragraph.

And so begins the start of the second paragraph but as you can see it's only marginally better than the other one so you've not really gained much - sorry. I am certainly not a budding author as you can see from this example input. Perhaps I need to work on my writing skills.

### Example 2 Output

(**150 072 626 840 312 999**) One hundred and fifty quadrillion, seventy-two trillion, six hundred and twenty-six billion, eight hundred and fourty million, three hundred and thirteen thousand subtract one is a rather large prime number which equals one to five if calculated modulo two to six respectively.

(**221 806 434 537 978 679**) (**Subscribe for more Useless Number Facts(tm)!**) However, one other rather more interesting number is two hundred and twenty-one quadrillion, eight hundred and six trillion, four hundred and thirty-four billion, five hundred and thirty-seven milmillion, nine hundred and seventy-eight thousand, six hundred and seventy nine, which isn't prime but is the 83rd Lucas number.

### Example 3 Output

(**Aha, now you are stumped! top kek**) (**Nothing to see here.**) Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex.

Duis aute irure dolor in repre-henderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

# Finally

Got any cool challenge ideas? Submit them to /r/DailyProgrammer_Ideas!
