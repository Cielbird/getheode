# Geþeode

*[Geþeode](https://en.wiktionary.org/wiki/geþeode)* `/jeˈθeoː.de/` is an Old English word meaning 
"language"

A Rust library that provides tools for building fictional languages. 

The library provides :
- A universal representation for phonological strings
- A way to apply phonological changes to words 

Future goals:
- A CLI
- Representation of phonotactic grammar
- Design a "phonological optimisation". after a set of sound changes, the definitions 
    for the phonology can be optimized to minimise the transformations used. both lossy and lossless
    algorithms could be used.
- Including machine learning
  - audio rendition of a language's words
  - training an LM on phonological strings

*Thanks to:*

Bruce Hayes and Eric Biggs for the ipa segment features csv data:
https://linguistics.ucla.edu/people/hayes/IP/#features

## Use

> Due to a re-write under way, the CLI is unusable now.

Run a set of phonological rules on a set of words :

`cargo run surface -b examples/romanization.txt -r examples/rules.txt -i examples/input.txt`

## Building

`cargo build`
