#!/bin/bash
# Plasmatic MX Message Parsing Library
# https://github.com/GoPlasmatic/MXMessage
#
# Copyright (c) 2025 Plasmatic
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Publish JSON Schema files and manifest to GitHub release
#
# Usage:
#   ./scripts/publish_release.sh <version>
#   ./scripts/publish_release.sh --dry-run
#
# Example:
#   ./scripts/publish_release.sh v3.1.4
#   ./scripts/publish_release.sh --dry-run

set -e

REPO="GoPlasmatic/MXMessage"
SCHEMAS_DIR="schemas"

# Check for dry-run mode
DRY_RUN=false
if [ "$1" == "--dry-run" ]; then
    DRY_RUN=true
    # Get version from Cargo.toml for dry-run
    VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
    TAG="v$VERSION"
    echo "=== DRY RUN MODE ==="
    echo "Would publish version: $TAG"
else
    TAG=$1
    if [ -z "$TAG" ]; then
        echo "Usage: $0 <version> or $0 --dry-run"
        echo "Example: $0 v3.1.4"
        exit 1
    fi
    VERSION=${TAG#v}  # Remove 'v' prefix if present
fi

echo "Publishing MXMessage release $TAG"
echo ""

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo "Error: GitHub CLI (gh) is not installed."
    echo "Install it from: https://cli.github.com/"
    exit 1
fi

# Check if authenticated
if ! gh auth status &> /dev/null; then
    echo "Error: Not authenticated with GitHub CLI."
    echo "Run: gh auth login"
    exit 1
fi

# Generate schemas
echo "Generating JSON schemas..."
cargo run --example generate_manifest --features jsonschema -- "$VERSION"

if [ ! -d "$SCHEMAS_DIR" ]; then
    echo "Error: Schemas directory not found at $SCHEMAS_DIR"
    exit 1
fi

# Count generated files
SCHEMA_COUNT=$(find "$SCHEMAS_DIR" -name "*.schema.json" | wc -l | tr -d ' ')
echo "Generated $SCHEMA_COUNT schema files"

if [ "$DRY_RUN" == "true" ]; then
    echo ""
    echo "=== DRY RUN - Files that would be uploaded ==="
    ls -la "$SCHEMAS_DIR"
    echo ""
    echo "=== DRY RUN - Release notes that would be created ==="
    cat << EOF
## MXMessage $VERSION

### Plugin Manifest & JSON Schemas

This release includes:
- \`manifest.json\` - Plugin manifest with all supported message types
- JSON Schema files for all 25 supported ISO20022 MX message types

### Supported Message Types

#### Payment Clearing & Settlement (PACS)
pacs.002, pacs.003, pacs.004, pacs.008, pacs.009, pacs.010

#### Payment Initiation (PAIN)
pain.001, pain.002, pain.008

#### Cash Management (CAMT)
camt.025, camt.029, camt.052, camt.053, camt.054, camt.055, camt.056, camt.057, camt.058, camt.060, camt.105, camt.106, camt.107, camt.108, camt.109

#### Administration (ADMI)
admi.024

### Usage

Download the manifest:
\`\`\`bash
curl -sL https://github.com/$REPO/releases/download/$TAG/manifest.json | jq .
\`\`\`

Download a specific schema:
\`\`\`bash
curl -sLO https://github.com/$REPO/releases/download/$TAG/pacs.008.001.08.schema.json
\`\`\`
EOF
    echo ""
    echo "=== DRY RUN COMPLETE ==="
    echo "Run without --dry-run to publish the release"
    exit 0
fi

# Check if tag exists
if ! git rev-parse "$TAG" >/dev/null 2>&1; then
    echo "Warning: Tag $TAG does not exist locally."
    echo "Make sure the tag exists on GitHub before running this script."
fi

# Check if release already exists
if gh release view "$TAG" --repo "$REPO" &> /dev/null; then
    echo "Release $TAG already exists. Uploading assets to existing release..."
    RELEASE_EXISTS=true
else
    echo "Creating new release $TAG..."
    RELEASE_EXISTS=false
fi

# Create release notes
RELEASE_NOTES=$(cat << EOF
## MXMessage $VERSION

### Plugin Manifest & JSON Schemas

This release includes:
- \`manifest.json\` - Plugin manifest with all supported message types
- JSON Schema files for all 25 supported ISO20022 MX message types

### Supported Message Types

#### Payment Clearing & Settlement (PACS)
pacs.002, pacs.003, pacs.004, pacs.008, pacs.009, pacs.010

#### Payment Initiation (PAIN)
pain.001, pain.002, pain.008

#### Cash Management (CAMT)
camt.025, camt.029, camt.052, camt.053, camt.054, camt.055, camt.056, camt.057, camt.058, camt.060, camt.105, camt.106, camt.107, camt.108, camt.109

#### Administration (ADMI)
admi.024

### Usage

Download the manifest:
\`\`\`bash
curl -sL https://github.com/$REPO/releases/download/$TAG/manifest.json | jq .
\`\`\`

Download a specific schema:
\`\`\`bash
curl -sLO https://github.com/$REPO/releases/download/$TAG/pacs.008.001.08.schema.json
\`\`\`
EOF
)

if [ "$RELEASE_EXISTS" == "false" ]; then
    echo "$RELEASE_NOTES" | gh release create "$TAG" \
        --repo "$REPO" \
        --title "MXMessage $VERSION" \
        --notes-file - \
        "$SCHEMAS_DIR"/*.json
else
    # Upload assets to existing release
    gh release upload "$TAG" \
        --repo "$REPO" \
        --clobber \
        "$SCHEMAS_DIR"/*.json
fi

echo ""
echo "=== Release published successfully ==="
echo "View release: https://github.com/$REPO/releases/tag/$TAG"
echo ""
echo "Quick test commands:"
echo "  curl -sL https://github.com/$REPO/releases/download/$TAG/manifest.json | jq '.supported_messages | length'"
echo "  curl -sLO https://github.com/$REPO/releases/download/$TAG/pacs.008.001.08.schema.json"
