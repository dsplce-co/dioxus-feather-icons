#!/bin/bash

# Directory containing the original .svg files
SOURCE_DIR="../src/icons/original"

for filepath in "$SOURCE_DIR"/*.svg; do
  # Get the filename without the directory and extension
  filename=$(basename "$filepath" .svg)

  cat > "../src/icons/${filename}.svg" <<EOF
<svg style="width:var(--size,24px);height:var(--size,24px)">
  <use href="#${filename}" />
</svg>
EOF
done

