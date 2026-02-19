---
description: Workflow for creating and managing project specifications and the specification registry.
---

# Specification Workflow

This workflow defines a universal, technology-agnostic process for creating and managing project specifications in the `docs/specifications/` directory. It is designed to be applicable across any stack (Frontend, Backend, Fullstack, GameDev, etc.).

## Directory Structure

The specification documentation follows this structure:

```plaintext
docs/
└── specifications/
    ├── INDEX.md                  # System file: Dispatcher & Registry
    ├── ROADMAP.md                # System file: Project Roadmap & Prioritization
    ├── {specification-name}.md   # Content file: Specific logic (e.g., architecture.md)
    └── ...
```

## Workflow Steps

1. **Context Analysis**: Determine the domain of the new specification.
2. **Directory Check**: Ensure `docs/specifications/` exists.
3. **System Files Verification**:
    - Check if `INDEX.md` and `ROADMAP.md` exist.
    - If missing, create them with initial structural content.
4. **Content Creation**:
    - Create the new `{specification-name}.md`.
5. **Registry Update**:
    - Add a reference to the new file in `INDEX.md`.
    - Update `ROADMAP.md` if the new specification introduces new features or milestones.

## Templates

Use these templates to ensure consistency across all specification files.

### 1. INDEX.md (System File)

- **Purpose**: Acts as a dispatcher/registry.
- **Content**: Aggregates information about all `{specification-name}.md` files and defines their logical relationships.

```markdown
# Specifications Registry

**Version:** {X.Y.Z}
**Status:** {Active}

---

## Overview

This index serves as the central registry for all project specifications, detailing their relationships and current status.

## System Files

- [ROADMAP.md](ROADMAP.md) - Global project roadmap and prioritization strategy.

## Domain Specifications

- [api-gateway.md](api-gateway.md) - API endpoints and authentication flow.
- [database-schema.md](database-schema.md) - SQL structure and migrations.
- [ui-system.md](ui-system.md) - Design system and component library.

---

## Meta Information

- **Maintainer**: Core Team
- **License**: MIT
- **Last Updated**: {YYYY-MM-DD}
```

### 2. ROADMAP.md (System File)

- **Purpose**: Defines the development plan and priorities.
- **Content Examples**:
  - **Phase 1: Core Foundation (P0)** (e.g. MVP, Basic Infrastructure)
  - **Phase 2: Resilience & Security (P1)** (e.g. Error Handling, Auth, Validation)
  - **Phase 3: Visual Polish & UX (P2)** (e.g. Optimizations, Animations)

```markdown
# Project Roadmap

**Version:** {X.Y.Z}
**Status:** {Active}

---

## Overview

Strategic development plan prioritizing core features, resilience, and user experience.

## Phase 1: MVP & Core Features (P0)

- **Feature A**: Core functionality implementation.
- **Feature B**: Basic infrastructure setup.

## Phase 2: Scalability & Optimization (P1)

- **Performance**: Caching layer implementation.
- **Security**: OAuth integration.

---

## Meta Information

- **Last Updated**: {YYYY-MM-DD}
- **Next Review**: {YYYY-MM-DD}
```

### 3. Specification File Template ({name}.md)

- **Naming**: Use lowercase, kebab-case. Examples:
  - `architecture.md` (System Design)
  - `api.md` (Interface Contracts)
  - `database-schema.md` (Data Layer)
  - `ui-components.md` (Frontend Design)
- **Purpose**: Detailed specifications for specific logical domains.

```markdown
# {Specification Name}

**Version:** {X.Y.Z}
**Status:** {Draft | RFC | Stable}

---

## Overview

Brief summary of the specification's purpose and scope.

## 1. Motivation

Why is this specification needed? What problems does it solve?

## 2. Detailed Design

### 2.1 Component A
Technical details, diagrams, and logic.

### 2.2 Component B
...

## 3. Drawbacks & Alternatives

Potential issues and alternative approaches considered.

---

## Document History

| Version | Date       | Author | Description       |
| :---    | :---       | :---   | :---              |
| 0.1.0   | YYYY-MM-DD | User   | Initial Draft     |
```
