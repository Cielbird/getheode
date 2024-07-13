# geþeode

*[geþeode](https://en.wiktionary.org/wiki/geþeode)* `/jeˈθeoː.de/` is the old english word for 
"language"

a rust library that provides tools for building fictional languages. the end 
goal is an engine that automates the process of conlanging (constructing ficitonal languages).
it will aim to mimic the patterns of real languages as well as possible.

for now the library will only be capable of working with language phonologies.
it will be able to generate or load a user-built phonology. it will be able to apply sound changes,
and "evolve" it, as if it were a real language changing in time.

this is an early version of a project I previously began developing in C#. I am also
still learning rust. 

future goals are
- design a "phonological optimisation". after a set of sound changes, the definitions 
    for the phonology can be optimized to minimise the transformations used. both lossy and lossless
    algorithms could be used.
- including machine learning
  - audio rendition of a language's words
  - an attempt at grammar and auto-translation

*thanks to:*

Bruce Hayes and Eric Biggs for the ipa segment features csv data:
https://linguistics.ucla.edu/people/hayes/IP/#features

## overview

The library provides tools to work with phonologies. a phonology is represented with the following 
components:
- phoneme inventory
- phonotactics
- rules

## file syntax

these components can be defined from files. these files have their own syntax.

comments can be added with `//` in any file.

### phonemes

the set of phonemes that exist in the language. each phoneme is a string that is matched with a
sound segment that is the phoneme's underlying representation 
([UR](https://en.wikipedia.org/wiki/Underlying_representation)).

each phoneme is written as:
```
[representation] = [ipa]
```

representations make it easier to read and write morphemes in the language you are creating. 
representations allow you 


Example:
```
a = æ
e = e
ee = iː
```

in cases where you wish the romanization to be the same as the IPA symbol, you can leave out the 
romanization:
```
a = æ
e
ee = iː
```

### rules
in the engine, "rules" refer to 
[phonological rules](https://en.wikipedia.org/wiki/Phonological_rule).

> phonological rules are commonly used in generative phonology as a notation to capture 
> sound-related operations and computations the human brain performs when producing or 
> comprehending spoken language.

the synax of a rules file is as follows. Each rule is written on it's own line:

```
[input] -> [output] / [pre-context]_[post-context] 
```

to represent a word border in the context, use: `#`

in the input and contexts, brackets and commas `{ , }` are used to represent **or**.

the following means "i **or** e becomes j, before an a **or** an o": 
```
{i, e} -> j / _{a, o}
```

`[input]`, `[output]`, `[pre-context]` and `[post-context]` are writen either 
using the IPA, or bracketted features: `[+voi ...]`

### phonotactics

phonotactics define how phonemes can or can't be arranged in morphemes.

### Morphemes

[what is a morpheme?](https://en.wikipedia.org/wiki/Morpheme) 

the morpheme file is simple compared to the others. morphemes to load are written on seperate 
lines, using the romanization of the phomenes defined in the phonemes file.


```
// nouns
gato
perro
perla

// adjectives
con
sobre
sin
```
