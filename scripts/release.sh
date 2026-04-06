#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "Usage: $0 <version> [-C <directory>]" >&2
  echo "Example: $0 0.2.0" >&2
  echo "         $0 0.2.0 -C /path/to/project" >&2
  exit 1
}

if [ $# -lt 1 ]; then
  usage
fi

VERSION="$1"
shift

# Parse optional -C flag
PROJECT_DIR=""
while [ $# -gt 0 ]; do
  case "$1" in
    -C)
      [ $# -lt 2 ] && usage
      PROJECT_DIR="$2"
      shift 2
      ;;
    *)
      usage
      ;;
  esac
done

# Change to project directory if specified
if [ -n "$PROJECT_DIR" ]; then
  cd "$PROJECT_DIR"
fi

TAG="v${VERSION}"

# Validate semver format
if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$'; then
  echo "Error: Invalid version format '${VERSION}'. Expected semver (e.g. 0.2.0)" >&2
  exit 1
fi

# Check Cargo.toml exists
if [ ! -f Cargo.toml ]; then
  echo "Error: Cargo.toml not found in $(pwd)" >&2
  echo "Use -C to specify the project directory." >&2
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
echo "Updated Cargo.toml to version ${VERSION}"
grep '^version' Cargo.toml

# Update npm package.json versions
if [ -d npm ]; then
  for pkg_json in npm/*/package.json; do
    node -e "
      const fs = require('fs');
      const pkg = JSON.parse(fs.readFileSync('${pkg_json}', 'utf8'));
      pkg.version = '${VERSION}';
      if (pkg.optionalDependencies) {
        for (const dep of Object.keys(pkg.optionalDependencies)) {
          pkg.optionalDependencies[dep] = '${VERSION}';
        }
      }
      fs.writeFileSync('${pkg_json}', JSON.stringify(pkg, null, 2) + '\n');
    "
    echo "Updated ${pkg_json} to version ${VERSION}"
  done
fi

# Commit and tag
git add Cargo.toml npm/
git commit -m "release: ${TAG}"
git tag "$TAG"

echo ""
echo "Created commit and tag '${TAG}'."
echo "Run 'git push && git push origin ${TAG}' to trigger the release workflow."
