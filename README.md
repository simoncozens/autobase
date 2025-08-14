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
