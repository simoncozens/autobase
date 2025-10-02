# Autobase

This is a utility for generating [BASE
tables](https://learn.microsoft.com/en-us/typography/opentype/spec/base) for OpenType fonts.

It has two modes:

1) If the font contains CJK glyphs, it generates a BASE table as per the
[CJK Vertical
Metrics](https://googlefonts.github.io/gf-guide/metrics.html#base-table)
requirements of Google Fonts.

2) Otherwise, it uses the
[fontheight](https://github.com/googlefonts/fontheight) library to
analyze the highest and lowest coordinates of shaped strings in the
various scripts supported by the font, and generates [script specific
MinMax
tables](https://learn.microsoft.com/en-us/typography/opentype/spec/base#minmax-extent-values)
to allow user agents to alter text vertical metrics based on the script
in use.

## Configuring the MinMax tables

`autobase` can be configured by passing a TOML configuration file to the `-c` argument. This TOML file can have the following keys:

* `languages` is a list of script-language combinations to be split out of the main calculation and handled separately.
* `overrides` is a dictionary of min and/or max values to be manually set for a particular script-language combination.
* `tolerance` is a number of font units within which language-specific MinMax values will be considered close enough to the script or font default to be omitted.

In both cases, script-language combinations are specified as `yyy_Xxxx` where `yyy` is a valid [ISO639-1](https://en.wikipedia.org/wiki/List_of_ISO_639_language_codes) or [ISO639-3](https://iso639-3.sil.org/code_tables/639/data) language code and `Xxxx` is a valid [ISO 15924 four-letter script code](https://www.unicode.org/iso15924/iso15924-codes.html).

An example will make this clear. The following config file:

```toml
tolerance=10
languages=["vi_Latn"]
[override]
fi_Latn = { max = 1234 }
```

with a font which supports Latin and Cyrillic will create:

* A default language system MinMax table for `Cyrl` based on values measured when shaping the Cyrillic word lists, unless the values are within 10 font units of the font's default (OS/2 typo ascender / descender).
* A language system entry for Finnish with the max value set to 1234 and the min value automatically computed from the Finnish word list - unless the values are within 10 font units of the Latin script default.
* A language system entry for Vietnamese with max and min values automatically computed, again unless the values are within 10 font units of the Latin script default.
* A default MinMax table for `Latn` based on shaping all Latin wordlists *except* Finnish and Vietnamese words, unless the values are within 10 font units of the font's default.