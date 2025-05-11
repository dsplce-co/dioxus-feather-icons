#!/bin/bash

OUTPUT_FILE="../src/icons/_sprite.svg"

# Empty the output file if it already exists
: > "$OUTPUT_FILE"

echo '<svg xmlns="http://www.w3.org/2000/svg" style="display:none">' >> "$OUTPUT_FILE"

for filepath in ../src/icons/original/*.svg; do
  filename=$(basename "$filepath" .svg)

   viewBox=$(grep -o 'viewBox="[^"]*"' "$filepath")

   # Replace <svg ...> with <symbol id="..." viewBox="..."> and </svg> with </symbol>
   sed -e 's/^<\?xml.*?>//' \
       -e "s|<svg[^>]*>|<symbol id=\"$filename\" $viewBox fill=\"currentColor\">|" \
       -e 's|</svg>|</symbol>|' "$filepath" >> "$OUTPUT_FILE"

  echo >> "$OUTPUT_FILE"
done

echo '</svg>' >> "$OUTPUT_FILE"
