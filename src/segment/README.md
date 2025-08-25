# Segment

In the context of the Geþeode library, "segment" usually has a particular meaning.

A segment is the smallest unit of sound in a language. Most segments can be represented with 
IPA characters. In the word "help", there are 4 segments: [h], [ɛ], [l] and [p].

## Features

In Getheode, segments are stored as a set of feature states. There are 28 features that can differentiate different segments. Each feature can be positive (+), negative (-), non-applicable, or undefined. 

Any sound in any human language can be differentiated via these 28 features. It is worth noting that the topic of features, and which should exist, is a debated subject among linguists.

The set of features used in Getheode are the following:

* Major class
  * `syl` - Sylabic
  * `stress` - Stressed
  * `long` - Long
  * `cons` - Consonantal
  * `son` - Sonorant
* Manner
  * `cont` - Continuant
  * `delrel` - Delayed release
  * `approx` - Approximant
  * `tap` - Tap
  * `trill` - Trill
  * `nasal` - Nasal
* Laryngeal
  * `voi` - Voiced
  * `spgl` Spread glottis
  * `congl` - Constricted glottis
* Place
  * `lab` - Labial
    * `round` - Rounded
    * `labdent` - Labial-dental
  * `cor` - Coronal
    * `ant` - Anterior
    * `dist` - Distributed
  * `strident` - Strident
  * `lateral` - Lateral
  * `dor` - Dorsal
    * `high`
    * `low`
    * `front`
    * `back`
  * `tense`

## Complete and incomplete segments

In Getheode, a segment can be *incomplete*. This means one or more features are *undefined*. This can be used for patern matching and transformations.

## Formatting

Segments can be serialized as strings in multiple ways:

### Pure IPA
  * `e`
  * `ɣ˕`

Diacritics are mostly supported.

### Feature sets
  * `[+voi-del]`
  * `[+tap]`

Feature sets are usually for incomplete segments, like when defining phonological rules.

### Natural phonological class

Classes are incompelete segments that are commonly identified in linguistics, like consonants or 
vowels. Classes are useful shorthands for common features sets. The supported classes are:

  * `C` - Consonant 
  * `V` - Vowel
