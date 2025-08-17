#!/usr/bin/env python3
"""
Post-processing script to apply Box<T> wrapping to prevent stack overflow in generated Rust code.
This script analyzes struct definitions and applies Box wrapping based on configurable criteria.
"""

import os
import re
import json
import argparse
from pathlib import Path
from typing import Dict, Set, List, Tuple, Optional
from dataclasses import dataclass, field
from collections import defaultdict

@dataclass
class BoxConfig:
    """Configuration for Box wrapping decisions"""
    # Specific types to always box
    always_box_types: Set[str] = field(default_factory=lambda: {
        "ReportEntry101", "ReportEntry102", "ReportEntry103",
        "EntryDetails91", "EntryDetails92", "EntryDetails93",
        "EntryTransaction101", "EntryTransaction102", "EntryTransaction103",
        "TransactionParties61", "TransactionParties62", "TransactionParties63",
        "TransactionAgents51", "TransactionAgents52", "TransactionAgents53",
        "RemittanceInformation161", "RemittanceInformation162", "RemittanceInformation163",
    })
    
    # Patterns for types to box when in Vec
    box_in_vec_patterns: List[str] = field(default_factory=lambda: [
        r".*Entry\d+$",           # Matches ReportEntry101, etc.
        r".*Transaction\d+$",     # Matches EntryTransaction101, etc.
        r".*Details\d+$",         # Matches EntryDetails91, etc.
        r".*Parties\d+$",         # Matches TransactionParties61, etc.
    ])
    
    # Box types in Vec if parent matches these patterns
    box_vec_if_parent_matches: List[str] = field(default_factory=lambda: [
        r".*Report\d+$",          # Report types often have deep nesting
        r".*Statement\d+$",       # Statement types often have deep nesting
    ])
    
    # Size thresholds
    min_fields_to_box: int = 10  # Box structs with more than N fields
    max_nesting_depth: int = 4   # Box if nesting depth exceeds this
    
    # Enable different strategies
    use_size_heuristic: bool = True
    use_pattern_matching: bool = True
    use_nesting_analysis: bool = True
    
    @classmethod
    def from_json(cls, json_path: str) -> 'BoxConfig':
        """Load configuration from JSON file"""
        with open(json_path, 'r') as f:
            data = json.load(f)
        return cls(**data)
    
    def to_json(self, json_path: str):
        """Save configuration to JSON file"""
        data = {
            'always_box_types': list(self.always_box_types),
            'box_in_vec_patterns': self.box_in_vec_patterns,
            'box_vec_if_parent_matches': self.box_vec_if_parent_matches,
            'min_fields_to_box': self.min_fields_to_box,
            'max_nesting_depth': self.max_nesting_depth,
            'use_size_heuristic': self.use_size_heuristic,
            'use_pattern_matching': self.use_pattern_matching,
            'use_nesting_analysis': self.use_nesting_analysis,
        }
        with open(json_path, 'w') as f:
            json.dump(data, f, indent=2)

class StructAnalyzer:
    """Analyzes Rust struct definitions to determine Box wrapping needs"""
    
    def __init__(self, config: BoxConfig):
        self.config = config
        self.structs: Dict[str, Dict[str, str]] = {}  # struct_name -> {field_name: field_type}
        self.struct_fields_count: Dict[str, int] = {}  # struct_name -> field count
        self.nesting_depth: Dict[str, int] = {}  # Cached nesting depths
        
    def parse_file(self, file_path: str) -> Dict[str, Dict[str, str]]:
        """Parse all structs from a file"""
        with open(file_path, 'r') as f:
            content = f.read()
        
        local_structs = {}
        
        # Find all struct definitions
        struct_pattern = r'pub struct (\w+)\s*\{([^}]*)\}'
        for match in re.finditer(struct_pattern, content, re.DOTALL):
            struct_name = match.group(1)
            struct_body = match.group(2)
            
            fields = {}
            field_count = 0
            
            # Parse fields
            field_pattern = r'pub\s+(\w+):\s+(.+?)(?:,|\n)'
            for field_match in re.finditer(field_pattern, struct_body):
                field_count += 1
                field_name = field_match.group(1)
                field_type = field_match.group(2).strip().rstrip(',')
                
                # Extract core type from Option<Vec<Type>> patterns
                core_type = self._extract_core_type(field_type)
                fields[field_name] = (field_type, core_type)
            
            local_structs[struct_name] = fields
            self.struct_fields_count[struct_name] = field_count
        
        return local_structs
    
    def _extract_core_type(self, field_type: str) -> str:
        """Extract the core type from Option<Vec<Type>> patterns"""
        # Remove Box wrapper if present
        field_type = re.sub(r'Box<(.+?)>', r'\1', field_type)
        
        # Handle Vec<Type>
        vec_match = re.search(r'Vec<(\w+)>', field_type)
        if vec_match:
            return vec_match.group(1)
        
        # Handle Option<Type>
        opt_match = re.search(r'Option<(\w+)>', field_type)
        if opt_match:
            return opt_match.group(1)
        
        # Direct type
        type_match = re.match(r'^(\w+)', field_type)
        if type_match:
            return type_match.group(1)
        
        return field_type
    
    def calculate_nesting_depth(self, struct_name: str, visited: Set[str] = None) -> int:
        """Calculate maximum nesting depth for a struct"""
        if struct_name in self.nesting_depth:
            return self.nesting_depth[struct_name]
        
        if visited is None:
            visited = set()
        
        if struct_name in visited:
            return 0  # Cycle detected
        
        visited.add(struct_name)
        
        max_depth = 0
        if struct_name in self.structs:
            for field_name, (field_type, core_type) in self.structs[struct_name].items():
                if self._is_primitive_type(core_type):
                    continue
                
                if core_type in self.structs:
                    child_depth = self.calculate_nesting_depth(core_type, visited.copy())
                    max_depth = max(max_depth, child_depth + 1)
        
        self.nesting_depth[struct_name] = max_depth
        return max_depth
    
    def _is_primitive_type(self, type_name: str) -> bool:
        """Check if a type is a primitive type"""
        primitives = {
            'String', 'bool', 'f64', 'f32', 'u64', 'i64', 'u32', 'i32', 
            'u16', 'i16', 'u8', 'i8', 'usize', 'isize', 'char'
        }
        return type_name in primitives
    
    def should_box_field(self, struct_name: str, field_name: str, field_type: str, core_type: str) -> bool:
        """Determine if a field should be boxed based on configuration"""
        
        # Never box primitive types
        if self._is_primitive_type(core_type):
            return False
        
        # Check if already boxed
        if 'Box<' in field_type:
            return False
        
        # Strategy 1: Always box specific types
        if core_type in self.config.always_box_types:
            return True
        
        # Strategy 2: Pattern matching for Vec types
        if self.config.use_pattern_matching and 'Vec<' in field_type:
            # Check type patterns
            for pattern in self.config.box_in_vec_patterns:
                if re.match(pattern, core_type):
                    return True
            
            # Check parent patterns
            for pattern in self.config.box_vec_if_parent_matches:
                if re.match(pattern, struct_name):
                    return True
        
        # Strategy 3: Size heuristic
        if self.config.use_size_heuristic:
            if core_type in self.struct_fields_count:
                if self.struct_fields_count[core_type] >= self.config.min_fields_to_box:
                    if 'Vec<' in field_type:  # Only box large types in Vec
                        return True
        
        # Strategy 4: Nesting depth analysis
        if self.config.use_nesting_analysis and 'Vec<' in field_type:
            if core_type in self.structs:
                depth = self.calculate_nesting_depth(core_type)
                if depth >= self.config.max_nesting_depth:
                    return True
        
        return False
    
    def apply_boxing(self, file_path: str, dry_run: bool = False) -> Tuple[int, List[str]]:
        """Apply Box wrapping to a file"""
        with open(file_path, 'r') as f:
            content = f.read()
        
        original_content = content
        changes = []
        
        # First pass: analyze all structs
        self.structs = self.parse_file(file_path)
        
        # Second pass: apply boxing
        for struct_name, fields in self.structs.items():
            for field_name, (field_type, core_type) in fields.items():
                if self.should_box_field(struct_name, field_name, field_type, core_type):
                    # Determine what to wrap
                    if 'Vec<' + core_type + '>' in field_type:
                        # Box items in Vec
                        old_pattern = f'Vec<{core_type}>'
                        new_pattern = f'Vec<Box<{core_type}>>'
                        
                        # Find and replace in struct definition
                        struct_pattern = rf'(pub struct {struct_name}\s*\{{[^}}]*pub\s+{field_name}:\s+)'
                        field_pattern = rf'{old_pattern}'
                        
                        # Build regex to find the exact field
                        full_pattern = rf'(pub struct {struct_name}\s*\{{[^}}]*pub\s+{field_name}:\s+\S*?){re.escape(old_pattern)}'
                        
                        if re.search(full_pattern, content, re.DOTALL):
                            new_content = re.sub(full_pattern, rf'\1{new_pattern}', content, flags=re.DOTALL)
                            if new_content != content:
                                content = new_content
                                changes.append(f"  {struct_name}.{field_name}: Vec<{core_type}> -> Vec<Box<{core_type}>>")
                    
                    elif field_type == core_type:
                        # Box single field
                        # Find and replace in struct definition
                        full_pattern = rf'(pub struct {struct_name}\s*\{{[^}}]*pub\s+{field_name}:\s+){re.escape(core_type)}(\s*[,\n])'
                        
                        if re.search(full_pattern, content, re.DOTALL):
                            new_content = re.sub(full_pattern, rf'\1Box<{core_type}>\2', content, flags=re.DOTALL)
                            if new_content != content:
                                content = new_content
                                changes.append(f"  {struct_name}.{field_name}: {core_type} -> Box<{core_type}>")
        
        # Handle validation code updates for boxed types
        if changes:
            content = self._update_validation_code(content, changes)
        
        if not dry_run and content != original_content:
            with open(file_path, 'w') as f:
                f.write(content)
        
        return len(changes), changes

    def _update_validation_code(self, content: str, changes: List[str]) -> str:
        """Update validation code to handle boxed types properly"""
        # The validation code should already handle Box<T> through Deref trait
        # But we might need to adjust iteration patterns
        
        # For Vec<Box<T>>, the iteration already works with for item in vec
        # Box implements Deref, so item.validate() will work
        
        return content

def main():
    parser = argparse.ArgumentParser(
        description='Apply Box<T> wrapping to prevent stack overflow in generated Rust code'
    )
    parser.add_argument('directory', help='Directory containing generated Rust files')
    parser.add_argument('--config', help='Path to JSON configuration file')
    parser.add_argument('--save-config', help='Save current configuration to JSON file')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be changed without modifying files')
    parser.add_argument('--verbose', action='store_true', help='Show detailed analysis')
    
    # Configuration overrides
    parser.add_argument('--min-fields', type=int, help='Override minimum fields threshold')
    parser.add_argument('--max-depth', type=int, help='Override maximum nesting depth')
    parser.add_argument('--no-patterns', action='store_true', help='Disable pattern matching')
    parser.add_argument('--no-size', action='store_true', help='Disable size heuristic')
    parser.add_argument('--no-nesting', action='store_true', help='Disable nesting analysis')
    
    args = parser.parse_args()
    
    # Load or create configuration
    if args.config:
        config = BoxConfig.from_json(args.config)
    else:
        config = BoxConfig()
    
    # Apply command-line overrides
    if args.min_fields:
        config.min_fields_to_box = args.min_fields
    if args.max_depth:
        config.max_nesting_depth = args.max_depth
    if args.no_patterns:
        config.use_pattern_matching = False
    if args.no_size:
        config.use_size_heuristic = False
    if args.no_nesting:
        config.use_nesting_analysis = False
    
    # Save configuration if requested
    if args.save_config:
        config.to_json(args.save_config)
        print(f"Configuration saved to {args.save_config}")
    
    # Process files
    analyzer = StructAnalyzer(config)
    dir_path = Path(args.directory)
    
    if not dir_path.is_dir():
        print(f"Error: {args.directory} is not a directory")
        return 1
    
    total_changes = 0
    modified_files = []
    
    print(f"Processing Rust files in {args.directory}...")
    if args.dry_run:
        print("DRY RUN MODE - No files will be modified\n")
    
    if args.verbose:
        print(f"Configuration:")
        print(f"  Always box types: {len(config.always_box_types)} types")
        print(f"  Pattern matching: {config.use_pattern_matching}")
        print(f"  Size heuristic: {config.use_size_heuristic} (threshold: {config.min_fields_to_box} fields)")
        print(f"  Nesting analysis: {config.use_nesting_analysis} (threshold: {config.max_nesting_depth} depth)")
        print()
    
    for file_path in sorted(dir_path.glob('*.rs')):
        if file_path.name == 'mod.rs':
            continue
        
        change_count, changes = analyzer.apply_boxing(str(file_path), dry_run=args.dry_run)
        
        if change_count > 0:
            total_changes += change_count
            modified_files.append(file_path.name)
            
            print(f"📄 {file_path.name}: {change_count} fields boxed")
            if args.verbose:
                for change in changes:
                    print(change)
    
    print(f"\n{'='*60}")
    print(f"Summary: {total_changes} fields boxed across {len(modified_files)} files")
    
    if modified_files and not args.dry_run:
        print(f"\nModified files:")
        for file_name in modified_files:
            print(f"  ✓ {file_name}")
        print(f"\nRun 'cargo build' to verify the changes compile correctly.")
    
    return 0

if __name__ == '__main__':
    exit(main())