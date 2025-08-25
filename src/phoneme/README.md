# Getheode phoneme

A phoneme is the atomic building block of words in a language. 

In Getheode, phonemes are a [segment](../segment/readme.md) with metadata.

## Formatting

A phoneme bank is formatted with lines of the following format:
```
<symbol>: <segment>
```
The symbol can be any string, excluding spaces.

The segment is writen with IPA.

If the segment is easy to write, you may just write:
```
<segment/symbol>
```
Where the symbol is also the ipa representation of the segment.
