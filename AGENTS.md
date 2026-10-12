> AI-generated content.

# Agent Instructions

## AI Provenance Labels

- Every Markdown file created by an AI assistant must include `> AI-generated content.` near the top, after any required front matter.
- Every test file created by an AI assistant must include a language-appropriate header, such as `// AI-generated tests.` for Rust.
- When modifying human-authored or mixed-authorship Markdown or tests, mark the AI-authored additions or edits explicitly. Use a file header such as `> Contains AI-generated additions or edits.` or `// Contains AI-generated tests or test edits.` when changes span the file; use adjacent comments when attribution can be localized.
- Preserve existing provenance labels. Do not describe unmodified human-authored content as AI-generated.
- These requirements also apply to test helpers, test-module scaffolding, documentation, and this file itself.
- Before finishing, verify that all Markdown and test files created or modified by the assistant carry an appropriate AI provenance label.
- Keep Markdown content in English.

## Design Requests

- Decline requests that conflict with project code rules or clearly impose unjustified performance costs or reduce readability. Explain the concrete problem respectfully and offer a better alternative.
- For viable requests with design flaws or meaningful trade-offs, use the question tool to agree on a better approach rather than rejecting or silently changing the design.

## Architecture Record

- Maintain `agent-instructions/Architect.md` as the project architecture record, including relevant files, responsibilities, key functions, and implementation limits.
- On agent initialization and after every architecture change, inspect the current source and create or update this record. Keep it consistent with implemented behavior, not planned features.
- The coding agent is responsible for this maintenance. Follow the Markdown provenance and English-language rules above.
