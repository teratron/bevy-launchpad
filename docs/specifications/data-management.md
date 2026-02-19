# Data Management & Assets

**Version:** 0.1.0
**Status:** Draft

---

## Overview

Specification for asset management, persistence, and application configuration.

## Related Specifications

- [architecture.md](architecture.md) - Infrastructure context.

## 1. Asset Orchestration (AAA Approach)

Система управляет тысячами ассетов через манифесты.

- **Manifest (`assets.ron`)**: Внешняя регистрация ресурсов.
- **Orchestrator**: Обработка зависимостей и предварительная компиляция для GPU.
- **Embedded Resources**: Резервные ассеты внутри бинарного файла.

## 2. Persistence (Save System)

- **Architecture**: Слой сериализации (Reflect/Serde).
- **Security**: Version Guard (миграция) и проверка целостности (Checksum).

## 3. Configuration & Infrastructure

### 3.1 Application Configuration

- **AppConfig**: Системные параметры (%APPDATA%).
- **Hot-Reloading**: Живое обновление параметров через `FileWatchers`.

### 3.2 Auto-Update Workflow

1. **Check**: Сравнение версий.
2. **Security**: Верификация подписи.
3. **Atomic Patch**: Применение в изолированном каталоге.
4. **Rollback**: Откат при сбое.

---

## Document History

| Version | Date       | Author | Description   |
| :---    | :---       | :---   | :---          |
| 0.1.0   | 2026-02-19 | Agent  | Initial Draft |
