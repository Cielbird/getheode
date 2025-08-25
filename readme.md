# Geþeode

*[Geþeode](https://en.wiktionary.org/wiki/geþeode)* `/jeˈθeoː.de/` is an Old English word meaning 
"language"

A Rust library that provides tools for building fictional languages. 

The end goal is an engine that automates the process of conlanging (constructing ficitonal languages).
it will aim to mimic the patterns of real languages as well as possible.

For now the library will only be capable of working with language phonologies.
It will be able to generate or load a user-built phonology. it will be able to apply sound changes,
and "evolve" it, as if it were a real language changing in time.

This is an early version of a project I previously began developing in C#. I am also
still learning Rust. 

Future goals are
- Design a "phonological optimisation". after a set of sound changes, the definitions 
    for the phonology can be optimized to minimise the transformations used. both lossy and lossless
    algorithms could be used.
- Including machine learning
  - audio rendition of a language's words
  - an attempt at grammar and auto-translation

*Thanks to:*

Bruce Hayes and Eric Biggs for the ipa segment features csv data:
https://linguistics.ucla.edu/people/hayes/IP/#features


## Building

`cargo build`

## Glossary

| term    | description                                                            |
| ------- | ---------------------------------------------------------------------- |
| segment | can be represented by a single IPA character. one discrete sound.      |
| lect    | a single way of speaking. this should not be confused with a language. |

A lect could be a particular dialect, sociolect, or even just a single person's way of speaking.
