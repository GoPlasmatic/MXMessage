#!/bin/bash

input_directory=./xsd
output_directory=./src
xgen_path=../xgen/cmd/xgen/xgen
cp header.rs $output_directory/common.rs

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

lib_file=$output_directory"/lib.rs"

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
' > "$lib_file"

for f in `find $output_directory -iname "*.rs" -type f -print | sort -n`; do
    module_name=$(basename "$f" .rs)
    if [ "$module_name" != "lib" ] && [ "$module_name" != "common" ]; then
        echo "pub mod $module_name;" >> "$lib_file"
    fi
done