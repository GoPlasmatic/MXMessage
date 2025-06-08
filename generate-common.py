import os
import re
from pathlib import Path
from collections import defaultdict
from dataclasses import dataclass
import argparse

@dataclass
class StructMatch:
    filename: str
    content: str
    start_pos: int
    end_pos: int
    usage_count: int = 0

def scan_rust_files(directory: str) -> tuple[defaultdict, dict, list]:
    """
    Scan all .rs files (except mod.rs) for struct and enum definitions and their usage in type definitions.
    Returns uppercase types for mod.rs and lowercase types for removal.
    """
    type_locations = defaultdict(list)
    file_contents = {}
    lowercase_matches = []
    
    # Pattern to find struct/enum definitions with their comments
    struct_pattern = re.compile(
        r'(// (\w+): [^\n]*\n'  # Capture comment with type name
        r'(?://[^\n]*\n)*'      # Optional additional comment lines
        r'#\[derive[^\]]*\]\n'  # Derive attribute
        r'pub (?:struct|enum) \2 \{[^}]*\})',  # Struct or enum definition
        re.MULTILINE | re.DOTALL
    )
    
    # Pattern to find impl blocks for validation
    impl_pattern = re.compile(
        r'(impl (\w+) \{[^}]*pub fn validate\(&self\) -> Result<\(\), ValidationError> \{.*?\n\})',
        re.MULTILINE | re.DOTALL
    )
    
    # Combined pattern for complete struct + impl removal
    complete_pattern = re.compile(
        r'// (\w+): [^\n]*\n'  # Comment with type name
        r'(?://[^\n]*\n)*'      # Optional additional comment lines
        r'#\[derive[^\]]*\]\n'  # Derive attribute
        r'pub (?:struct|enum) \1 \{[^}]*\}\n+'  # Struct or enum definition
        r'(?:impl \1 \{[^}]*pub fn validate\(&self\) -> Result<\(\), ValidationError> \{.*?\n\}\n*)?',  # Optional impl block with validation
        re.MULTILINE | re.DOTALL
    )
    
    # Usage pattern - look for type names in field definitions
    usage_pattern = re.compile(r'pub \w+: (?:Option<)?(?:Vec<)?(\w+)(?:>)?(?:>)?,?')
    
    dir_path = Path(directory)
    if not dir_path.is_dir():
        print(f"Directory {directory} does not exist")
        return type_locations, file_contents, lowercase_matches
    
    # Get all .rs files except mod.rs
    rust_files = [f for f in os.listdir(dir_path) 
                  if f.endswith('.rs') and f != 'mod.rs']
    
    print(f"Found {len(rust_files)} .rs files: {', '.join(rust_files)}")
    
    # Process each file to get complete content
    for filename in rust_files:
        file_path = dir_path / filename
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            file_contents[filename] = content
        except Exception as e:
            print(f"Error reading {filename}: {e}")
    
    # Now find struct definitions and their impl blocks
    for filename in rust_files:
        content = file_contents[filename]
        
        # Find all struct/enum definitions
        struct_matches = list(struct_pattern.finditer(content))
        impl_matches = list(impl_pattern.finditer(content))
        
        print(f"Found {len(struct_matches)} type definitions in {filename}")
        
        # Create a map of impl blocks by type name
        impl_map = {}
        for impl_match in impl_matches:
            impl_type_name = impl_match.group(2)
            impl_map[impl_type_name] = impl_match.group(1)
        
        for struct_match in struct_matches:
            type_name = struct_match.group(2)
            print(f"  - {type_name}")
            
            # Combine struct definition with its impl block if it exists
            complete_content = struct_match.group(1)
            if type_name in impl_map:
                complete_content += "\n\n" + impl_map[type_name]
            
            struct_match_obj = StructMatch(
                filename=filename,
                content=complete_content,
                start_pos=struct_match.start(),
                end_pos=struct_match.end()
            )
            type_locations[type_name].append(struct_match_obj)
    
    # Count usage of each type across all files
    for type_name in type_locations.keys():
        total_usage = 0
        for filename, content in file_contents.items():
            usage_matches = usage_pattern.findall(content)
            usage_count = usage_matches.count(type_name)
            total_usage += usage_count
        
        # Set usage count for all matches of this type
        for struct_match in type_locations[type_name]:
            struct_match.usage_count = total_usage
    
    return type_locations, file_contents, lowercase_matches

def generate_mod_file(duplicate_types: dict, output_file: str):
    """Generate or update mod.rs file with duplicate types."""
    existing_types, existing_content = read_existing_mod(output_file)
    
    # Prepare headers for mod.rs if it's a new file
    if not existing_content:
        headers = """// Plasmatic MX Message Parsing Library
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

use crate::error::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

"""
        existing_content = headers
    
    # Prepare all content before writing to file
    new_content = [existing_content.rstrip('\n')]
    
    # Add new types
    for type_name, matches in sorted(duplicate_types.items()):
        if type_name not in existing_types:
            new_content.append(matches[0].content.rstrip('\n'))
    
    new_content.append('\n')
    # Write all content at once
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write('\n'.join(new_content))

def remove_duplicates_from_files(duplicate_types: dict, file_contents: dict, dir_path: Path):
    """Remove duplicate type definitions from original files."""
    
    # Combined pattern for complete struct + impl removal
    def create_removal_pattern(type_name):
        return re.compile(
            rf'// {type_name}: [^\n]*\n'  # Comment with type name
            r'(?://[^\n]*\n)*'            # Optional additional comment lines
            rf'#\[derive[^\]]*\]\n'       # Derive attribute
            rf'pub (?:struct|enum) {type_name} \{{[^}}]*\}}\n+'  # Struct or enum definition
            rf'(?:impl {type_name} \{{[^}}]*pub fn validate\(&self\) -> Result<\(\), ValidationError> \{{.*?\n\}}\n*)?',  # Optional impl block
            re.MULTILINE | re.DOTALL
        )
    
    for type_name, matches in duplicate_types.items():
        pattern = create_removal_pattern(type_name)
        
        for match in matches:
            filename = match.filename
            if filename in file_contents:
                # Remove the complete pattern (struct + impl) from file content
                file_contents[filename] = pattern.sub('', file_contents[filename])
        
        print(f"Removed {type_name} from {len(matches)} files")
    
    # Write updated content back to files
    for filename, content in file_contents.items():
        if filename != 'mod.rs':  # Don't overwrite mod.rs
            file_path = dir_path / filename
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)

def print_summary(type_locations: defaultdict, lowercase_matches: list, typecount: int):
    """Print summary of type usage and lowercase types to be removed."""
    # Print lowercase types that will be removed
    if lowercase_matches:
        print("\nLowercase types to be removed:")
        print("-" * 40)
        by_file = defaultdict(list)
        for match in lowercase_matches:
            type_match = re.search(r'// (\w+) \.\.\.\n', match.content)
            if type_match:
                by_file[match.filename].append(type_match.group(1))
        
        for filename, types in sorted(by_file.items()):
            print(f"{filename}:")
            for type_name in sorted(types):
                print(f"  - {type_name}")
        print()
    
    # Identify root structs that should stay in their original files
    root_structs = set()
    for filename in set(match.filename for matches in type_locations.values() for match in matches):
        # Extract the base name from filename (e.g., camt_057_001_06.rs -> camt_057_001_06)
        if filename.endswith('.rs'):
            file_base = filename[:-3]  # Remove .rs extension
            
            for type_name in type_locations.keys():
                # Root structs are typically the main message types that:
                # 1. End with version numbers (V01, V02, etc.)
                # 2. Are document or message containers
                # 3. Are only defined in one file
                # 4. Have names that relate to the file structure
                if (type_name.endswith(('V01', 'V02', 'V03', 'V04', 'V05', 'V06', 'V07', 'V08', 'V09', 'V10')) or
                    'DOCUMENT' in type_name.upper() or
                    'MESSAGE' in type_name.upper() or
                    len(type_locations[type_name]) == 1):  # Types that appear in only one file
                    
                    # Check if this type is only in this specific file
                    if len(type_locations[type_name]) == 1 and type_locations[type_name][0].filename == filename:
                        root_structs.add(type_name)

    print(f"\nIdentified root structs to keep in original files: {', '.join(sorted(root_structs))}")
    
    # Print uppercase types summary - focus on types that appear in multiple files, excluding root structs
    frequent_types = {
        name: matches 
        for name, matches in type_locations.items() 
        if name not in root_structs and len(matches) > 1  # Only move types that appear in multiple files
    }
    
    if not frequent_types:
        print(f"No uppercase types found that appear in multiple files (excluding root structs).")
        return
    
    print(f"\nUppercase types that appear in multiple files (excluding root structs):")
    print("-" * 70)
    
    # Pre-calculate usage counts
    usage_data = [
        (type_name, matches, sum(m.usage_count for m in matches))
        for type_name, matches in frequent_types.items()
    ]
    
    # Sort by number of files first, then by total usage count
    for type_name, matches, total_usage in sorted(
        usage_data, key=lambda x: (len(x[1]), x[2]), reverse=True
    ):
        files = [match.filename for match in matches]
        print(f"{type_name}: appears in {len(files)} files, used {total_usage} times total")
        for match in matches:
            print(f"  - {match.filename}: {match.usage_count} uses")
    print()
    
    return frequent_types

def read_existing_mod(output_file: str) -> tuple[set, str]:
    """Read existing mod.rs file if it exists."""
    try:
        with open(output_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Compile pattern once
        existing_structs = set(re.findall(r'// (\w+) \.\.\.\n', content))
        return existing_structs, content
    except FileNotFoundError:
        return set(), ""

def main():
    parser = argparse.ArgumentParser(
        description='Find frequently used structs in .rs files and move to mod.rs'
    )
    parser.add_argument('directory', 
                       help='Directory containing .rs files (default: current directory)',
                       default='.',
                       nargs='?')

    parser.add_argument('typecount', 
                       help='Type count threshold (default: 1)',
                       default=1,
                       nargs='?')

    args = parser.parse_args()
    
    try:
        # Scan files and collect both uppercase and lowercase types
        type_locations, file_contents, lowercase_matches = scan_rust_files(args.directory)
        
        # Print summary and get frequent types
        frequent_types = print_summary(type_locations, lowercase_matches, int(args.typecount))
        
        if frequent_types or lowercase_matches:
            output_path = Path(args.directory) / 'mod.rs'
            
            # Handle uppercase types
            if frequent_types:
                # Read existing types
                seed_types, _ = read_existing_mod(output_path)
                if seed_types:
                    print(f"\nFound {len(seed_types)} existing types in mod.rs")
                
                # Process files
                generate_mod_file(frequent_types, output_path)
                new_types = set(frequent_types.keys()) - seed_types
                
                # Print results
                if new_types:
                    print(f"Added {len(new_types)} new types to mod.rs")
                    print("New types:", ", ".join(sorted(new_types)))
                else:
                    print("No new types to add")
            
            # Remove both duplicate uppercase types and lowercase types
            remove_duplicates_from_files(frequent_types, file_contents, Path(args.directory))
            if lowercase_matches:
                print(f"Removed {len(lowercase_matches)} lowercase types from original files")
            if frequent_types:
                print("Removed duplicate types from original files")
        
    except Exception as e:
        print(f"Error: {e}")
        return 1
    
    return 0

if __name__ == "__main__":
    exit(main())