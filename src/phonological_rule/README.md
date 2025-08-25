# Getheode phonological rules

## Formatting

```
<input> -> <output>
```

Inputs may have multiple options by using curly braces `{}` and commas `,`. The following pattern means, *"`input_1` or `input_2` becomes `output`"*

```
{<input_1>, <input_2>} -> {output}
```

To add context to the rule:

```
<inputs> -> <outputs> / <pre-context>_<post-context>
```

For the rule to apply, the pre and post contexts must match. 
Pre-context comes before segments on whihc the rule is applied, and post-context comes after.  

Context patterns may have multiple options:

```
<inputs> -> <outputs> / {<pre_1>, <pre_1>}_{<post_1>, <post_1>}
```

