# Phonological Rule Parsing

This is a pretty complicated parsing algorithm. There are pretty tiny edge cases that 
change the whole game.

1) Parse the patterns

The pattern tree is built. There is one tree for each:
- input
- output
- pre-context
- post-context

Example : `a{t, b}(i)`

The pattern tree is composed of the following nodes:

- Branches (in brackets for example : {a, b, c})
- Optionals (in parentheses for example: (a:) )
- Nulls (usually written with this symbol in text: Ø)
- Sequences, containing sequences of elements (example: abc)

We should have parsed the input as a `Vec<Pattern>`, the output as a set of `Vec<String>`, 
and the contexts as `Vec<Pattern>`.

2) Enumerate patterns

The possibilities of patterns need to be enumerated. This produces a list of rules. We still have 
raw strings.

Here, we need to build a tree of the different patterns. We *need to* enumerate all possibilities
before parsing the phonological segments. This is because, for example, `a(:)` is a valid string.
That is a vowel with a modifying diacritic that is optional (that is, after a branch). It gets 
pretty complicated if you want to represent lower-than-segment branching logic within a 
phonological string tree.

3) Parse elements of each pattern

The elements (segments, boundaries) of each input, output, and context can be parsed. `Strings` 
become `ElementSequences`

Elements can be either:
- Features (an ipa symbol, a natural class... including segment AND syllable features. delimit one feature)
- Boundaries (Syllable, Word, Segment...)
  (ambiguous boundaries like syllable/word are treated like a branch leading to other boundaries)

4) Input/Output Tagging

Context, input and output are tagged at this point. Each non-defined segment is tagged in order.
Tags written in text are parsed too.

The non-defined and still untagged segments are tagged.

The following simple algo is used:

The number of segments to tag should be the same on left and right. They are tagged in order.

5) Rule tree synthesis

A tree can now be built. Using the boundary and segment elements.

This element sequence :

`a$b#cd`

Becomes :
```
x
| \
o  o  <-- word level
| \  \
o  o   o   <-- syllable level
|  |   | \
a  b   c  d
```

At this point, we have a list of [PhonoRule]s.
