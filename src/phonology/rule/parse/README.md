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
- Leaf : containing raw string
- Nulls (usually written with this symbol in text: Ø)
- Sequences, containing sequences of elements (example: abc)

The tree still represents raw text, leaf nodes contain strings. 

We should have parsed the input as a `Vec<Pattern>`, the output as a set of `Vec<String>`, 
and the contexts as `Vec<Pattern>`.

1) Enumerate patterns

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

5) Tagging

Once syllables features are determined, we can fill in missing tags in the input/output as well
as give tags to the context trees.

4) Tree synthesis

Context is merged, so we have just input and output element sequences.

The element sequences are synthesized as trees. Syllables infos are merge.. etc.

- Be carefull here !
  when merging two nodes in the input for example, 
  we need to make sure to re-tag the same nodes on the output, so a tagged output node always 
  points to an exiting input node
