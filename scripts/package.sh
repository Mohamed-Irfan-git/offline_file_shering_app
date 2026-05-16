#!/usr/bin/env bash
# Build a portable LAN Share folder for Linux / macOS.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/dist/lan-share"
BIN_NAME="lan-share"

echo "==> Building release binary..."
cd "$ROOT"
cargo build --release

echo "==> Packaging to $OUT"
rm -rf "$OUT"
mkdir -p "$OUT/static" "$OUT/uploads"

cp "$ROOT/target/release/$BIN_NAME" "$OUT/"
cp -r "$ROOT/static/"* "$OUT/static/"

cat > "$OUT/run.sh" << 'EOF'
#!/usr/bin/env bash
cd "$(dirname "$0")"
echo "Starting LAN Share..."
echo "Open http://localhost:5000 on this computer."
./lan-share
EOF
chmod +x "$OUT/run.sh" "$OUT/$BIN_NAME"

cat > "$OUT/README.txt" << 'EOF'
LAN Share — portable package
============================

1. Connect this computer to Wi-Fi.
2. Double-click run.sh (or run ./lan-share in this folder).
3. Open http://localhost:5000 on this PC.
4. On phones/tablets: use the http://192.168.x.x:5000 address shown
   in the terminal (NOT localhost).

Requires: none (binary included). Firewall may ask to allow port 5000.
EOF

echo ""
echo "Done. Portable app:"
echo "  $OUT"
echo ""
echo "Zip example:"
echo "  cd dist && zip -r lan-share-$(uname -s)-$(uname -m).zip lan-share"
