# Localization System

**Version:** 0.1.0
**Status:** Draft

---

## Overview

Localization-First: весь текст интерфейса и системных сообщений загружается из внешних файлов Fluent (.ftl). Поддержка динамической смены языка.

## Related Specifications

- [architecture.md](architecture.md) - Infrastructure context.

## 1. Detailed Design

### 1.1 Project Fluent Integration

- **Grammar Support**: Нативная поддержка плюрализации.
- **Key-based Access**: Уникальные ID (напр., `lp.menu.start`).
- **Namespace Splitting**: Префиксы `lp.*` для либы и `game.*` для игры.

### 1.2 Regional Assets

- **Audio Locales**: Поддержка региональных аудиофайлов.
- **Fallback**: Автоматический откат к English (en-US).

---

## Document History

| Version | Date       | Author | Description   |
| :---    | :---       | :---   | :---          |
| 0.1.0   | 2026-02-19 | Agent  | Initial Draft |
