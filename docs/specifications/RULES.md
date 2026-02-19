# Project Specification Rules

**Version:** 1.0.0
**Status:** Active

---

## Overview

This file is the constitution of the specification system for this project.
It defines standing rules and conventions that apply to all spec files.
It is read by the agent before every operation and updated only via explicit triggers.

---

## 1. Naming Conventions

- Spec files use lowercase kebab-case: `api.md`, `database-schema.md`, `ui-components.md`.
- System files use uppercase: `INDEX.md`, `ROADMAP.md`, `RULES.md`.
- Section names within specs are title-cased.

## 2. Status Rules

A spec may only change status when the following criteria are met:

- **Draft → RFC**: the spec is complete enough for review; all required sections are filled.
- **RFC → Stable**: the spec has been reviewed and approved; no open questions remain.
- **Any → Deprecated**: the spec has been explicitly superseded; a replacement must be named.

## 3. Versioning Rules

- `patch` (0.0.X): typo fixes, wording clarifications — no structural or content change.
- `minor` (0.X.0): new section added, or existing section meaningfully extended.
- `major` (X.0.0): structural restructure, or a decision that changes the spec's scope.

## 4. Formatting Rules

- Use `plaintext` blocks for all directory trees.
- Use `mermaid` blocks for all flow diagrams and architecture diagrams.
- Do not use other diagram formats.

## 5. Content Rules

- No implementation code in spec files (no Rust, JS, Python, SQL, etc.).
- Pseudo-code and logic flows are permitted where necessary.
- Every spec must have an Overview, Motivation, and Related Specifications section.

## 6. Relations Rules

- Every spec that depends on another must declare it in `Related Specifications`.
- Cross-file content duplication is not permitted — use a link instead.
- Circular dependencies between specs must be flagged and resolved.

## 7. Project Conventions

- **Language**: All documentation must be in **English/Russian** mixed mode (Russian for content, English for structure/headers is acceptable as per current state).
- **Paths**: Use `assets/` relative paths in documentation.

---

## Document History

| Version | Date       | Author | Description              |
| :---    | :---       | :---   | :---                     |
| 1.0.0   | 2026-02-19 | Agent  | Initial constitution     |
