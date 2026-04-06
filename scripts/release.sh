#!/usr/bin/env bash
set -euo pipefail

if [ $# -ne 1 ]; then
  echo "Usage: $0 <version>" >&2
  echo "Example: $0 0.2.0" >&2
  exit 1
fi

VERSION="$1"
TAG="v${VERSION}"

# Validate semver format
if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$'; then
  echo "Error: Invalid version format '${VERSION}'. Expected semver (e.g. 0.2.0)" >&2
  exit 1
fi

# Check for uncommitted changes
if ! git diff --quiet || ! git diff --cached --quiet; then
  echo "Error: Working tree has uncommitted changes. Commit or stash them first." >&2
  exit 1
fi

# Check tag doesn't already exist
if git rev-parse "$TAG" >/dev/null 2>&1; then
  echo "Error: Tag '${TAG}' already exists." >&2
  exit 1
fi

# Update Cargo.toml version
sed -i '' "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml

# Verify the change
echo "Updated Cargo.toml to version ${VERSION}"
grep '^version' Cargo.toml

# Commit and tag
git add Cargo.toml
git commit -m "release: ${TAG}"
git tag "$TAG"

echo ""
echo "Created commit and tag '${TAG}'."
echo "Run 'git push && git push origin ${TAG}' to trigger the release workflow."
