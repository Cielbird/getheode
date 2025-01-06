# gbnf: getheode's Backus–Naur form

`gbnf` is a syntax for defining phonotactic structure with getheode

losely adapted this lib to segment strings: https://docs.rs/bnf/latest/bnf/#output

## syntax

there are two kinds of terms: terminal and non-terminal

terminal terms denote a strng of ipa segments that cannot be reduced further:
```
[aet]
[tata]
```

non-terminal terms are symbols that are defined else-where.
```
<vowel>
<sylable>
<coda>
```

a GBNF form file is a set of productions defined the following way:

```
<word> ::= <word_part><word_part>
<word_part> ::= <sylable><word_part> | <sylable>
<vowel> ::= [a] | [e] | [i] | [o] | [u]
<consonant> ::= [p] | [b] | [t] | [d] | [k] | [g]
<coda> ::= <consonant> | []
<sylable> ::= <consonant> <vowel> <coda>
```

the previous form can generate the following words (with `generate_random_word`):
```
kikobtidi
topupɡebpotbi
kudo
padaddutɡikutkate
petkopipɡiɡ
batbudpedipa
ɡedede
ɡoɡpa
bipupoba
pibbikbadidtodukbab
```
