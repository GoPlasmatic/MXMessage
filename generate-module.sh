#!/bin/bash

input_directory=$1
output_directory=$2
xgen_path=../xgen/cmd/xgen/xgen

# Replace dashes in the filenames with underscores
for f in `find $input_directory -iname "*.xsd" -type f -print`; do
    mv "$f" "${f//-/_}"
done

$xgen_path -i "$input_directory" -o "$output_directory" -l Rust -p iso20022

# Clean up unwanted files like .DS_Store.rs
for f in `find "$output_directory" -iname "*DS_Store.rs" -type f -print`; do
    rm -rf "$f"
done

# Rename .xsd.rs files to .rs
for f in `find "$output_directory" -iname "*.xsd.rs" -type f -print`; do
    mv "$f" "${f%.xsd.rs}.rs"
done

# Rename files with '.' in the file names to use '_', excluding the extension
find "$output_directory" -type f -name "*.rs" | while read -r file; do
    dir=$(dirname "$file")
    base_name=$(basename "$file" .rs | sed 's/\./_/g')  # Replace '.' with '_' only in the file name
    mv "$file" "$dir/$base_name.rs"  # Append the .rs extension correctly
done

mod_file=$output_directory"/mod.rs"

echo '// Plasmatic MX Message Parsing Library
// https://github.com/GoPlasmatic/MXMessage
//
// Copyright (c) 2025 Plasmatic
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// You may obtain a copy of this library at
// https://github.com/GoPlasmatic/MXMessage

' > "$mod_file"

for f in `find $output_directory -iname "*.rs" -type f -print | sort -n`; do
    module_name=$(basename "$f" .rs)
    if [ "$module_name" != "lib" ] && [ "$module_name" != "mod" ]; then
        echo "pub mod $module_name;" >> "$mod_file"
    fi
done

# Remove problematic document and app_hdr struct patterns from all .rs files
echo "Removing problematic document and app_hdr struct patterns from all .rs files..."
for f in `find $output_directory -iname "*.rs" -type f -print`; do
    echo "Processing: $f"
    # Use sed to remove the document struct pattern
    sed -i '' '/^\/\/ document \.\.\./,/^}$/d' "$f"
    # Remove the empty impl block that follows
    sed -i '' '/^impl document {$/,/^}$/d' "$f"

    # Replace app_hdr with AppHdr
    sed -i '' 's/app_hdr {/AppHdr {/g' "$f"

    # Replace serde rename AppHdr with flatten
    sed -i '' 's/#\[serde(rename = "AppHdr")\]/#\[serde(flatten)\]/g' "$f"
   
done

# python3 generate-common.py $output_directory
