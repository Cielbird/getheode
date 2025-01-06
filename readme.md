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


## building

`cargo build`
