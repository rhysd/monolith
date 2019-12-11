#!/bin/bash

set -e

if [ ! -d .git ]; then
    echo 'This script must be run at root directory of this repository' >&2
    exit 1
fi

if ! git diff --quiet; then
    echo 'Working tree is dirty! Please ensure all changes are committed and working tree is clean' >&2
    exit 1
fi

if ! git diff --cached --quiet; then
    echo 'Git index is dirty! Please ensure all changes are committed and Git index is clean' >&2
    exit 1
fi

branch="$(git symbolic-ref --short HEAD)"
if [[ "$branch" != "wasm" ]]; then
    echo "Current branch is not 'wasm'. Please move to 'wasm' before running this script" >&2
    exit 1
fi

sha="$(git rev-parse HEAD)"

echo "Releasing ${sha} to wasm-release branch"

set -x
rm -rf pkg
wasm-pack build --release
cp LICENSE ./pkg/

git switch wasm-release

srcs=(monolith.d.ts monolith.js monolith_bg.d.ts monolith_bg.wasm package.json LICENSE snippets/)
for src in "${srcs[@]}"; do
    cp -R "pkg/$src" "$src"
done

git add "${srcs[@]}"
git commit -m "the release built at ${sha}"
set +x

echo "Done. Please check 'git show HEAD' to verify changes. If ok, push this branch to remote"
