# TODO

- write up a semi-formal spec for the library to reorganize my thoughts

- manifest: add description

- Feature Rhyme: strict & approx, rhyme, alliteration, assonance, similarity

- Feature Meter: fitting syllables to breve and macron, feet?, moras?

- Reconsider structure for IPA symbols and relation to accents, phonemes.
  There should be an accent-independent, one-to-one correspondence between IPA symbols and phoneme information.
  Accents can map symbols to phonemes and reduce the set of phonemes for an accent, but this should be secondary
  and beyond the representation of IPA symbols in the library.
  Also consider the way that IPA symbols can be modified and relate to each other (aspirated consonant: 'p' and 'pʰ')
  An IPA representation that is limited to a single character without modifiers cannot represent all IPA symbols.
