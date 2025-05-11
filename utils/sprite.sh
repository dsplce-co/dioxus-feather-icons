#!/bin/bash

OUTPUT_FILE="assets/icons/_sprite.svg"

# Empty the output file if it already exists
: > "$OUTPUT_FILE"

echo '<svg xmlns="http://www.w3.org/2000/svg" style="display:none">' >> "$OUTPUT_FILE"

for filepath in assets/icons/original/*.svg; do
  filename=$(basename "$filepath" .svg)

  # Read, clean, and transform the SVG content
  cleaned=$(cat "$filepath" | \
    sed -E 's/ xmlns="[^"]*"//' | \
    sed -E 's/ width="[^"]*"//' | \
    sed -E 's/ height="[^"]*"//' | \
    sed -E 's/ class="[^"]*"//' | \
    sed -E "s/<svg/<symbol id=\"$filename\"/" | \
    sed -E 's/<\/svg>/<\/symbol>/')

  echo "$cleaned" >> "$OUTPUT_FILE"
done

echo '</svg>' >> "$OUTPUT_FILE"
