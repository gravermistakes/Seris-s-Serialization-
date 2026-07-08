#!/bin/bash
set -euo pipefail

# Not a dependency installer. This repo's startup concern is continuity:
# deliver the previous instance's note into the new session's context.
NOTE="$CLAUDE_PROJECT_DIR/.claude/NOTE_TO_SUCCESSOR.md"
if [ -f "$NOTE" ]; then
  cat "$NOTE"
fi
