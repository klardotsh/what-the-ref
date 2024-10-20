# What The Ref?! - augmented FIRST game manuals for referees

FIRST game manuals that bridge the gap between "color-coded cheat sheets" and
the full blown manual, with better accessibility than the PDFs to boot.
Maintained by [Josh Klar](//klar.sh), co-Senior Head Referee for FTC in
Washington State, with input from plenty more folks.

[See it online at WhatTheRef.info](//whattheref.info)

## Helpful Notes For Contributing

Rules, which are stored on disk as Markdown files, use a special link
destination to indicate interlinks (things which should link within What The
Ref to other things in the manual, and not to an external site), `!!`. For
example, `[ROBOT](!!)` would link to the glossary term for `ROBOT`, and
`[G404](!!)` would link to rule G404 in the in-match rules.

When converting FIRST rules from the manual into Markdown format, setting up
all these interlinks can be an exhausting process, so I recommend some clever
find-and-replace to help scriptify it. For example, in Vim, assuming I have a
visual mode selection of the lines of real rule text (eg. not the header or the
frontmatter), something like the following would automatically interlink any
matching strings, saving me tons of typing:

```
:'<,'>s/\(DRIVE TEAM\)/\[\1\]\(!!\)/gI
```

Hard-wrap your Markdown at 80 characters, please!

## Legal Blah-Blah-Blah

All rules content, images, media, and the branding of the games themselves,
are all copyright [FIRST Robotics](https://firstinspires.org). The licensing
of these resources is unclear, but my use of them here is intended as fair use
as a volunteer member of the program, to improve the consistency of refereeing
(and the quality of referee lives) across the program. Please don't come at me,
FIRST, there are bigger fish to fry.

All original text, imagery, etc., including the prose intepretations of
some rules, are licensed as closely to public domain as I can legally
muster in all jurisdictions, [Creative Commons Zero 1.0
Universal](https://creativecommons.org/publicdomain/zero/1.0/deed.en).

All code in this repository is likewise licensed as closely to public domain as
possible in the form of the [Zero-Clause BSD
License](https://opensource.org/license/0bsd/).

Don't use this code, prose, or any resources generated by them, for nefarious
purposes. This is a K12 education program for goodness' sake, be a good person.
