---
description: Discuss and document the project architecture and structure in `docs/architecture.md`.
---

This workflow is designed for architectural discussion and documentation. It strictly prohibits writing or proposing code.

1. **Initial Analysis**:
    - Use the `sequentialthinking` tool to analyze the current project structure and any existing architectural decisions.
    - Check for the existence of `docs/architecture.md`.
    - **AAA Alignment**: Reference and draw inspiration from established architectural patterns and best practices used in AAA game projects (e.g., modular systems, robust state management, efficient resource handling).

2. **Exploration & Brainstorming**:
    - Use the `brainstorming` skill to explore architectural patterns, component relationships, and system flows.
    - If the project involves specific frameworks (like Bevy), align with their best practices (e.g., ECS patterns).

3. **User Discussion**:
    - Discuss the architectural concepts with the user in Russian.
    - Ask clarifying questions about system requirements, integration points, and high-level design.

4. **Documentation**:
    - **Intelligent Updates**: Before modifying `docs/architecture.md`, thoroughly analyze the current content. Integrate new information logically: decide if it should be an addition, an update to an existing part, or a complete replacement of outdated sections to ensure document integrity.
    - Document the agreed-upon architecture in `docs/architecture.md`.
    - Use **English** for technical terms, symbols, and core descriptions.
    - **Russian Notes**: Use **Russian** for detailed explanations, notes, comments, and annotations within the documentation to provide clearer context for the Russian-speaking team.
    - **Folder Structures**: Include visual directory trees (e.g., using `plaintext` or `ascii` blocks) to describe the project layout and module organization.
    - Include **Mermaid diagrams** for visual representation of:
        - System components and their interactions.
        - Data flow and state machines.
        - Entity-Component relationships (for Bevy).
        - Module hierarchies.

5. **Code Restriction**:
    - **CRITICAL**: Do NOT write, propose, or generate any Rust, HTML, or other source code.
    - Focus exclusively on high-level design, traits interfaces (description only), and structural organization.

6. **Verification**:
    - Ensure `docs/architecture.md` is internally consistent and follows the project's design principles.
