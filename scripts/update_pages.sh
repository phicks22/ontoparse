#!/usr/bin/env bash
# Requires mdbook and mdbook-toc
set -euo pipefail

mdbook build book

worktree=$(mktemp -d)
git worktree add --force "$worktree" pages >/dev/null

rsync -a --delete --exclude '.git' book/book/ "$worktree"/
printf 'book/\ntarget/\n' > "$worktree/.gitignore"

git -C "$worktree" add -A
if git -C "$worktree" diff --cached --quiet; then
  echo "No changes to publish."
else
  git -C "$worktree" commit -m "Publish docs $(date -u +%Y-%m-%d)"
  git -C "$worktree" push origin pages
  echo "Published to https://phicks-it.codeberg.page/ontoparse/"
fi

git worktree remove --force "$worktree"
