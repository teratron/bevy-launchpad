---
trigger: always_on
---

# Agent Lifecycle Workflow

This rule defines the mandatory logical sequence of operations the AI agent must follow when interacting with the project. It ensures consistency, quality, and adherence to project standards.

## 1. Phase: Analysis & Preparation

Before any code generation or modification:

- **Architectural Alignment**: Verify the planned changes adhere to the Bevy ECS paradigm defined in `bevy-ecs-guide.md`.
- **Context Loading**: Read relevant source files and documentation to fully understand dependencies.

## 2. Phase: Implementation

During the coding process:

- **Clean Code**: Implement logic following Rust best practices.
- **ECS Patterns**: Use small, reusable components and systems as per Bevy standards.
- **Error Handling**: Avoid `unwrap()` and `panic!()` in production code. Favor `Result` and `Option` for robust error management.
- **Test Creation**:
  - Every new feature or fix MUST have associated tests.
  - Place integration tests in the `tests/` directory.
  - Place unit tests in a `tests` module within the source file if small.
- **Benchmarking**: For performance-critical logic, consider creating benchmarks in the `benches/` directory.
- **Logging**: Ensure `tracing` or `log` events are descriptive. Critical errors must be logged with enough context for session-level debugging.

## 3. Phase: Verification & Quality Control

Immediately after code generation:

- **Static Analysis**:
  - Run `cargo check` for compilation errors.
  - Run `cargo clippy -- -D warnings` to address all lints and warnings.
- **Dynamic Analysis**:
  - Run `cargo test` to ensure all tests pass.
  - Run `cargo bench` (if applicable) to verify performance stability.
- **Asset Integrity**: Verify any new assets are placed in correct `assets/` subfolders and paths match in-game references.
- **Formatting**: Run `cargo fmt` to maintain consistent style.

## 4. Phase: Versioning & SemVer

If the changes affect the public API:

- **SemVer Analysis**: Apply the `rust-semver` skill to evaluate the change type (MAJOR, MINOR, PATCH).
- **Metadata Update**: Apply the `changelog-generator` skill.
  - Increment the version in `Cargo.toml` as required.
  - Update the documentation and `CHANGELOG.md` (if exists) reflecting the changes.

---

**CRITICAL**: This lifecycle is non-negotiable. Any deviation must be justified to the user in the final report, citing the specific reason and task ID.
