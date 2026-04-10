# Phonological Rule Parsing

This is a pretty complicated parsing algorithm.

1) Parse element tree synthesis

The parse element tree is built. There is one tree for each:
- input
- output
- pre-context
- post-context

The parse element tree is composed of the following nodes:

- Branches (in brackets for example : {a, b, c})
- Optionals (in parentheses for example: (a:) )
- Nulls (usually written with this symbol in text: Ø)
- Sequences, containing sequences of elements (example: abc)

Elements can be either:
- Features (an ipa symbol, a natural class... including segment AND syllable features. delimit one feature)
- Boundaries (Syllable, Word, Segment...)
  (ambiguous boundaries like syllable/word are treated like a branch leading to other boundaries)

Context is tagged at this point. Each non-defined segment is tagged in order.
Tags written in text are parsed too.

At this point, we have a [ParsedRule].

2) Branch enumeration

Each leaf of each tree (input, output, prectx, postctx) is enumerated, and all combinations are 
listed. 

> Should context be appended to input and output here ? result would be only input->output ?

At this point, we have a list of [ParsedRule]s.

3) Input/Output Tagging

The non-defined and still untagged segments are tagged.

The following simple algo is used:

The number of segments to tag should be the same on left and right. They are tagged in order.

At this point, we have a list of [ParsedRule]s.

4) Rule tree synthesis

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

## Improvements

The big issue with this method is that we enumerate each possible match. The ideal step would be 
having a "tree" matching abstract syntax tree. That is, the branching logic would end up in the 
final pattern matching structure.  
