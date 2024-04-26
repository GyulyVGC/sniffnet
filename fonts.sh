#!/bin/zsh

# Script used to subset the font files to only include the necessary glyphs

FULL_FOLDER=resources/fonts/full
SUBSET_FOLDER=resources/fonts/subset
CHARACTERS_FILE=subset_characters.txt
FONT_NAME=sarasa-mono-sc

grep -o --no-filename . ./src/**/* | sort -u > $FULL_FOLDER/$CHARACTERS_FILE

pyftsubset $FULL_FOLDER/$FONT_NAME-bold.ttf --text-file=$FULL_FOLDER/$CHARACTERS_FILE --no-ignore-missing-unicodes
pyftsubset $FULL_FOLDER/$FONT_NAME-regular.ttf --text-file=$FULL_FOLDER/$CHARACTERS_FILE --no-ignore-missing-unicodes

mv $FULL_FOLDER/$FONT_NAME-bold.subset.ttf $SUBSET_FOLDER/$FONT_NAME-bold.subset.ttf
mv $FULL_FOLDER/$FONT_NAME-regular.subset.ttf $SUBSET_FOLDER/$FONT_NAME-regular.subset.ttf
