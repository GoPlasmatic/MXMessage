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
    Scan .rs files for struct and enum definitions and their usage in type definitions.
    Returns uppercase types for common.rs and lowercase types for removal.
    """
    type_locations = defaultdict(list)
    file_contents = {}
    lowercase_matches = []
    
    # Compile regex patterns once
    type_pattern = re.compile(
        r'(\n\n// (\w+) \.\.\.\n'
        r'#\[derive\(Debug, Default, Serialize, Deserialize, Clone, PartialEq\)\]\n'
        r'(?:pub struct|pub enum)\s+\w+.*?\t\tOk\(\(\)\)\n\t\}\n\}\n)',
        re.DOTALL
    )
    
    # More efficient base pattern for type usage
    base_usage_pattern = r'(?m)^[^/]*?:\s*(?:{}|Option<{}>|Vec<{}>|Option<Vec<{}>>)\s*,'
    base_usage_pattern = r'(?m)^[^/]*?:\s*([\w\d]+|Option<[\w\d]+>|Vec<[\w\d]+>|Option<Vec<[\w\d]+>>)\s*,'
    type_usage_patterns = re.compile(base_usage_pattern)

    dir_path = Path(directory).resolve()
    rust_files = [f for f in os.listdir(dir_path) if f.endswith('.rs')]
    
    # Process each file only once
    for filename in rust_files:
        file_path = dir_path / filename
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                file_contents[filename] = content
                
                # Find all type definitions in this file
                type_matches = list(type_pattern.finditer(content))
                
                def extract_type(type_str):
                    base_type = re.search(r'(\w+)(?:>)*,?$', type_str)
                    return base_type.group(1) if base_type else None

                type_usages = []
                for match in type_usage_patterns.finditer(content):
                    type_str = match.group(1)
                    base_type = extract_type(type_str)
                    if base_type:
                        type_usages.append(base_type)
                
                # Process each type definition
                for match in type_matches:
                    type_name = match.group(2)
                    struct_match = StructMatch(
                        filename=filename,
                        content=match.group(1),
                        start_pos=match.start(),
                        end_pos=match.end(),
                        usage_count=0
                    )
                    
                    if type_name[0].isupper():
                        # For uppercase types, count usages and add to type_locations
                        struct_match.usage_count = type_usages.count(type_name)
                        type_locations[type_name].append(struct_match)
                    else:
                        # For lowercase types, add to lowercase_matches for removal
                        lowercase_matches.append(struct_match)
                    
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
            
    return type_locations, file_contents, lowercase_matches

def generate_common_file(duplicate_types: dict, output_file: str):
    """Generate or update common.rs file with duplicate types."""
    existing_types, existing_content = read_existing_common(output_file)
    
    # Prepare all content before writing to file
    if not existing_content:
        existing_content = ""
    new_content = [existing_content.rstrip('\n')]
    
    # Add new types
    for type_name, matches in sorted(duplicate_types.items()):
        if type_name not in existing_types:
            new_content.append(matches[0].content.rstrip('\n'))
    
    new_content.append('\n')
    # Write all content at once
    with open(output_file, 'w', encoding='utf-8') as f:
        f.write('\n'.join(new_content))

def remove_duplicates(file_contents: dict, duplicate_structs: dict, lowercase_matches: list, directory: str):
    """Remove duplicate structs and lowercase structs from original files."""
    # Collect all positions to remove for each file
    file_positions = defaultdict(list)
    
    # Add positions from duplicate structs
    for matches in duplicate_structs.values():
        for match in matches:
            file_positions[match.filename].append((match.start_pos, match.end_pos))
    
    # Add positions from lowercase matches
    for match in lowercase_matches:
        file_positions[match.filename].append((match.start_pos, match.end_pos))
    
    # Process each file
    dir_path = Path(directory)
    for filename, positions in file_positions.items():
        if positions:
            content = file_contents[filename]
            # Sort positions in reverse order
            positions.sort(reverse=True)
            
            # Apply all changes at once
            new_content = content
            for start, end in positions:
                new_content = new_content[:start] + new_content[end:]
            
            # Write only if content changed
            if new_content != content:
                with open(dir_path / filename, 'w', encoding='utf-8') as f:
                    f.write(new_content)

def print_summary(type_locations: defaultdict, lowercase_matches: list, typecount: int):
    """Print summary of type usage and lowercase types to be removed."""
    # Print lowercase types that will be removed
    if lowercase_matches:
        print("\nLowercase types to be removed:")
        print("-" * 40)
        by_file = defaultdict(list)
        for match in lowercase_matches:
            by_file[match.filename].append(re.search(r'// (\w+) \.\.\.', match.content).group(1))
        
        for filename, types in sorted(by_file.items()):
            print(f"{filename}:")
            for type_name in sorted(types):
                print(f"  - {type_name}")
        print()
    
    # Print uppercase types summary
    frequent_types = {
        name: matches 
        for name, matches in type_locations.items() 
        if sum(match.usage_count for match in matches) >= typecount
    }
    
    if not frequent_types:
        print(f"No uppercase types found with usage count >= {typecount}.")
        return
    
    print(f"\nUppercase types with usage count >= {typecount}:")
    print("-" * 40)
    
    # Pre-calculate usage counts
    usage_data = [
        (type_name, matches, sum(m.usage_count for m in matches))
        for type_name, matches in frequent_types.items()
    ]
    
    # Sort by total usage count
    for type_name, matches, total_usage in sorted(
        usage_data, key=lambda x: x[2], reverse=True
    ):
        files = [match.filename for match in matches]
        print(f"{type_name}: used {total_usage} times across {len(files)} files")
        for match in matches:
            print(f"  - {match.filename}: {match.usage_count} uses")
    print()
    
    return frequent_types

def read_existing_common(output_file: str) -> tuple[set, str]:
    """Read existing common.rs file if it exists."""
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
        description='Find frequently used structs and move to common.rs'
    )
    parser.add_argument('directory', 
                       help='Directory containing .rs files (default: current directory)',
                       default='.',
                       nargs='?')

    parser.add_argument('typecount', 
                       help='Type count threashold (default: 1)',
                       default=1,
                       nargs='?')

    args = parser.parse_args()
    
    try:
        # Scan files and collect both uppercase and lowercase types
        type_locations, file_contents, lowercase_matches = scan_rust_files(args.directory)
        
        # Print summary and get frequent types
        frequent_types = print_summary(type_locations, lowercase_matches, int(args.typecount))
        
        if frequent_types or lowercase_matches:
            output_path = Path(args.directory) / 'common.rs'
            
            # Handle uppercase types
            if frequent_types:
                # Read existing types
                seed_types, _ = read_existing_common(output_path)
                if seed_types:
                    print(f"\nFound {len(seed_types)} existing types in common.rs")
                
                # Process files
                generate_common_file(frequent_types, output_path)
                new_types = set(frequent_types.keys()) - seed_types
                
                # Print results
                if new_types:
                    print(f"Added {len(new_types)} new types to common.rs")
                    print("New types:", ", ".join(sorted(new_types)))
                else:
                    print("No new types to add")
            
            # Remove both duplicate uppercase types and lowercase types
            remove_duplicates(file_contents, frequent_types, lowercase_matches, args.directory)
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