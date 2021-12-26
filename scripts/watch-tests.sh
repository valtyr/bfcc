#!/bin/bash
set -euxo pipefail
cd "$(git rev-parse --show-toplevel)"
find . -name '*.rs' | entr bash -c "clear && cargo test"
