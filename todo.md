# todo

Future goals:
- A CLI
- Representation of phonotactic grammar
- Design a "phonological optimisation". after a set of sound changes, the definitions 
    for the phonology can be optimized to minimise the transformations used. both lossy and lossless
    algorithms could be used.
- Including machine learning
  - audio rendition of a language's words
  - training an LM on phonological strings

## eventually
- fn to assert feature set validity (NA could be replaced by UNDEF, when a feature is no longer 
  applicable it is rendered UNDEF) 
- improve segment diacritic printing
- use cfg crate for grammar
- add phoneme definitions for simple word definition
- phonemes, phonotactics, and word generations with BNF
- packing lect changes in lect change nodes
- saving data to files
- add support for X-SAMPA and refactor accordingly
  - use of IPA and X-SAMPA should be inter-exchangable: 
  - ipa should be denoted with [] and xsampa with "" or other
- use this to assign sylable boundaries: http://glottopedia.org/index.php/Maximal_Onset_Principle

## maybe
- use phonotactic production labels in phonological rules
- get random sound changes from online databases
- generate sound changes with ml
- synthesize strings with ml
- web app tool
