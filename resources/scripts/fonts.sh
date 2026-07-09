#!/bin/zsh

# Script used to subset the font files to only include the necessary glyphs

FULL_FOLDER=resources/fonts/full
SUBSET_FOLDER=resources/fonts/subset
CHARACTERS_FILE=subset_characters.txt
FONT_NAME=sarasa-mono-sc-regular

grep -o --no-filename . ./src/**/* | sort -u > $FULL_FOLDER/$CHARACTERS_FILE

pyftsubset $FULL_FOLDER/$FONT_NAME.ttf --text-file=$FULL_FOLDER/$CHARACTERS_FILE --no-ignore-missing-unicodes

mv $FULL_FOLDER/$FONT_NAME.subset.ttf $SUBSET_FOLDER/$FONT_NAME.subset.ttf

# Subset Noto Sans Sinhala (OFL) to cover the Sinhala Unicode block and common
# zero-width joiners used for conjunct rendering.
# Font source: https://github.com/notofonts/noto-fonts (SIL Open Font License 1.1)
SINHALA_FONT=NotoSansSinhala-Regular
pyftsubset $FULL_FOLDER/$SINHALA_FONT.ttf \
    --unicodes="U+0D80-0DFF,U+200C,U+200D,U+25CC" \
    --output-file=$SUBSET_FOLDER/$SINHALA_FONT.subset.ttf
