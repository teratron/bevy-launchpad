# Development Guidelines

## Technical Stack

- **Rust**: Version 1.93 or higher.
- **Bevy**: Version 0.18 or higher.
- **Dependencies**: Use external dependencies only in exceptional cases. Maximize the use of the Rust standard library and Bevy built-in features before considering third-party crates.

## Windows-specific

- **Terminal**: When executing commands on Windows, always use uppercase drive letters in paths (e.g., `D:\...` instead of `d:\...`) to avoid issues with some tools.

## Related Documentation

- [Rules](.agent/rules/) - Additional development rules and guidelines
- [Skills](.agent/skills/) - Specialized capabilities and tools for development tasks
- [Workflows](.agent/workflows/) - Automated workflows and automation scripts
