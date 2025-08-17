rm -rf src/document/
rm -rf src/header/
sh generate-module.sh xsd/document/ src/document/
sh generate-module.sh xsd/header/ src/header/

# Apply Box wrapping to prevent stack overflow issues
echo "Applying Box wrapping to prevent stack overflow..."
python3 apply-box-wrapping.py src/document/ --config box-config.json

cargo clippy --fix --allow-dirty
cargo fmt
