#!/bin/zsh

# Script used to subset the font files to only include the necessary glyphs

FULL_FOLDER=resources/fonts/full
SUBSET_FOLDER=resources/fonts/subset
CHARACTERS_FILE=subset_characters.txt
FONT_NAME=sarasa-mono-sc-regular

grep -o --no-filename . ./src/**/* | sort -u > $FULL_FOLDER/$CHARACTERS_FILE

pyftsubset $FULL_FOLDER/$FONT_NAME.ttf --text-file=$FULL_FOLDER/$CHARACTERS_FILE --no-ignore-missing-unicodes

mv $FULL_FOLDER/$FONT_NAME.subset.ttf $SUBSET_FOLDER/$FONT_NAME.subset.ttf
