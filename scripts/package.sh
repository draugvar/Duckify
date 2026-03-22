#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────────────────
# package.sh  –  Build Duckify.app + Duckify-<version>.dmg
#
# Usage:
#   ./scripts/package.sh              # builds and packages
#   ./scripts/package.sh --sign       # also code-signs + notarizes
#
# Requirements:
#   - Rust toolchain
#   - create-dmg  (brew install create-dmg)
#   - For --sign: Apple Developer ID cert + notarytool credentials in env
# ─────────────────────────────────────────────────────────────────────────────
set -euo pipefail

VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')
APP_NAME="Duckify"
BUNDLE="${APP_NAME}.app"
DMG="${APP_NAME}-${VERSION}.dmg"
SIGN=false

for arg in "$@"; do
  [[ "$arg" == "--sign" ]] && SIGN=true
done

echo "▶ Building Duckify v${VERSION}..."
cargo build --release

echo "▶ Assembling ${BUNDLE}..."
rm -rf "${BUNDLE}"
mkdir -p "${BUNDLE}/Contents/MacOS"
mkdir -p "${BUNDLE}/Contents/Resources"

cp target/release/duckify          "${BUNDLE}/Contents/MacOS/duckify"
cp assets/Info.plist               "${BUNDLE}/Contents/Info.plist"
cp assets/icon.icns                "${BUNDLE}/Contents/Resources/icon.icns"

chmod +x "${BUNDLE}/Contents/MacOS/duckify"

# ── Code signing ──────────────────────────────────────────────────────────────
if [[ "${SIGN}" == true ]]; then
  : "${DEVELOPER_ID:?Set DEVELOPER_ID to your 'Developer ID Application: ...' cert name}"
  : "${APPLE_ID:?Set APPLE_ID to your Apple ID email}"
  : "${APPLE_APP_PASSWORD:?Set APPLE_APP_PASSWORD to your app-specific password}"
  : "${TEAM_ID:?Set TEAM_ID to your Apple Developer Team ID}"

  echo "▶ Code-signing..."
  codesign --deep --force --options runtime \
    --timestamp \
    --sign "${DEVELOPER_ID}" \
    "${BUNDLE}"

  echo "▶ Creating zip for notarization..."
  ditto -c -k --sequesterRsrc --keepParent "${BUNDLE}" "${APP_NAME}-notarize.zip"

  echo "▶ Submitting for notarization (this may take a few minutes)..."
  xcrun notarytool submit "${APP_NAME}-notarize.zip" \
    --apple-id "${APPLE_ID}" \
    --password "${APPLE_APP_PASSWORD}" \
    --team-id  "${TEAM_ID}" \
    --wait

  echo "▶ Stapling notarization ticket..."
  xcrun stapler staple "${BUNDLE}"
  rm -f "${APP_NAME}-notarize.zip"

  echo "✔ Signed and notarized."
fi

# ── DMG ───────────────────────────────────────────────────────────────────────
echo "▶ Creating ${DMG}..."
rm -f "${DMG}"

if command -v create-dmg &>/dev/null; then
  create-dmg \
    --volname "${APP_NAME}" \
    --volicon "assets/icon.icns" \
    --window-pos 200 120 \
    --window-size 540 380 \
    --icon-size 128 \
    --icon "${BUNDLE}" 140 190 \
    --hide-extension "${BUNDLE}" \
    --app-drop-link 400 190 \
    "${DMG}" \
    "${BUNDLE}"
else
  # Fallback: plain hdiutil
  hdiutil create -volname "${APP_NAME}" \
    -srcfolder "${BUNDLE}" \
    -ov -format UDZO \
    "${DMG}"
fi

# ── SHA256 ────────────────────────────────────────────────────────────────────
SHA=$(shasum -a 256 "${DMG}" | awk '{print $1}')
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  ✔  ${DMG}"
echo "  SHA256: ${SHA}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Next steps:"
echo "  1. Upload ${DMG} to GitHub Releases as v${VERSION}"
echo "  2. Copy the SHA256 above into your Homebrew tap formula"
