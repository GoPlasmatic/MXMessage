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
    Scan camt_*.rs and pacs_*.rs files for struct and enum definitions and their usage in type definitions.
    Returns uppercase types for common.rs and lowercase types for removal.
    """
    type_locations = defaultdict(list)
    file_contents = {}
    lowercase_matches = []
    
    # Compile regex patterns once
    type_pattern = re.compile(
        r'(// (\w+) \.\.\.\n'
        r'#\[derive\(Debug, Default, Serialize, Deserialize, Clone, PartialEq\)\]\n'
        r'(?:pub struct|pub enum)\s+\w+.*?'
        r'impl \w+ \{\s*pub fn validate\(&self\) -> Result<\(\), ValidationError> \{.*?\n\s*\}\s*\n\})',
        re.DOTALL
    )
    
    # Pattern for type usage in field definitions
    type_usage_patterns = re.compile(r'pub \w+: ([\w\d]+|Option<[\w\d]+>|Vec<[\w\d]+>|Option<Vec<[\w\d]+>>)\s*,')

    dir_path = Path(directory).resolve()
    # Filter for only camt_*.rs and pacs_*.rs files
    rust_files = [f for f in os.listdir(dir_path) 
                  if f.endswith('.rs') and (f.startswith('camt_') or f.startswith('pacs_'))]
    
    if not rust_files:
        print(f"No camt_*.rs or pacs_*.rs files found in {directory}")
        return defaultdict(list), {}, []
    
    print(f"Found {len(rust_files)} camt_/pacs_ files: {', '.join(sorted(rust_files))}")
    
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
                    # Extract base type from Option<Type>, Vec<Type>, Option<Vec<Type>>, or just Type
                    base_type = re.search(r'(?:Option<(?:Vec<)?)?(\w+)(?:>)?>?', type_str)
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
        # Extract the message type from filename (e.g., camt_057_001_06.rs -> camt.057.001.06)
        if filename.startswith(('camt_', 'pacs_')):
            # Look for structs that match the file pattern (these are usually the root message structs)
            file_base = filename.replace('.rs', '').replace('_', '').upper()
            for type_name in type_locations.keys():
                # Root structs often contain the file identifier or are main message types
                type_upper = type_name.upper()
                if (file_base[:4] in type_upper or  # e.g., CAMT in CamtXXXXXXVXX
                    type_name.endswith(('V01', 'V02', 'V03', 'V04', 'V05', 'V06', 'V07', 'V08', 'V09')) or
                    'PROPRIETARYMESSAGE' in type_upper or
                    'DOCUMENT' in type_upper):
                    # Check if this type is only in one file (root structs shouldn't be duplicated)
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
        description='Find frequently used structs in camt_*.rs and pacs_*.rs files and move to common.rs'
    )
    parser.add_argument('directory', 
                       help='Directory containing camt_*.rs and pacs_*.rs files (default: current directory)',
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