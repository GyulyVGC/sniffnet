#!/bin/zsh

# Script used to subset the font files to only include the necessary glyphs

FULL_FOLDER=resources/fonts/full
SUBSET_FOLDER=resources/fonts/subset
CHARACTERS_FILE=subset_characters.txt
FONT_NAME_REGULAR=GoNotoKurrent-Regular
FONT_NAME_BOLD=GoNotoKurrent-Bold

grep -o --no-filename . ./src/translations/**/* | sort -u > $FULL_FOLDER/$CHARACTERS_FILE

pyftsubset $FULL_FOLDER/$FONT_NAME_BOLD.ttf --text-file=$FULL_FOLDER/$CHARACTERS_FILE --no-ignore-missing-unicodes
pyftsubset $FULL_FOLDER/$FONT_NAME_REGULAR.ttf --text-file=$FULL_FOLDER/$CHARACTERS_FILE --no-ignore-missing-unicodes

mv $FULL_FOLDER/$FONT_NAME_BOLD.subset.ttf $SUBSET_FOLDER/$FONT_NAME_BOLD.subset.ttf
mv $FULL_FOLDER/$FONT_NAME_REGULAR.subset.ttf $SUBSET_FOLDER/$FONT_NAME_REGULAR.subset.ttf
