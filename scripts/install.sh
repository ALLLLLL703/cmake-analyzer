#!/usr/bin/env bash
# AI-authored installer.
set -euo pipefail

project_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

cargo install \
    --path "$project_root" \
    --bin cmake-analyzer \
    --root "$HOME/.local" \
    --locked \
    --force

printf '\nInstalled: %s/.local/bin/cmake-analyzer\n' "$HOME"
case ":${PATH:-}:" in
    *":$HOME/.local/bin:"*) ;;
    *) printf 'Add %s/.local/bin to your PATH.\n' "$HOME" ;;
esac
