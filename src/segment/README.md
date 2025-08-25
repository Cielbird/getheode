# Segment

In the context of the Geþeode library, "segment" usually has a particular meaning.

A segment is the smallest unit of sound in a language. Most segments can be represented with 
IPA characters. In the word "help", there are 4 segments: [h], [ɛ], [l] and [p].

## Features

In Getheode, segments are stored as a set of feature states. There are 28 features that can differentiate different segments. Each feature can be positive (+), negative (-), non-applicable, or undefined. 

Any sound in any human language can be differentiated via these 28 features. It is worth noting that the topic of features, and which should exist, is a debated subject among linguists. 

The set of features used in Getheode are the following:

* Major class
  * SYL
  * STRESS
  * LONG
  * CONS
  * SON
* Manner
  * CONT
  * DELREL
  * APPROX
  * TAP
  * TRILL
  * NASAL
* Laryngeal
  * VOI
  * SPGL
  * CONGL
* Place
  * LABIAL
  * LAB
  * ROUND
  * LABDENT
* Coronal
  * COR
  * ANT
  * DIST
  * STRIDENT
  * LATERAL
* Dorsal
  * DOR
  * HIGH
  * LOW
  * FRONT
  * BACK
  * TENSE

## Complete and incomplete segments

In Getheode, a segment can be *incomplete*. This means one or more features are *undefined*. This can be used for patern matching and transformations.

## Formatting

Segments can be serialized as strings in multiple ways:

* Pure IPA
  * `example`
* IPA and/or features (usually with incomplete segments)
  * `exampl[+voi]`
