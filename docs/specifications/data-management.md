# Data Management & Assets

**Version:** 0.3.0
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
- **Base Path Detection**: Надежное определение директории `assets/` при помощи `std::env::current_exe()` для обеспечения работы приложения при прямом запуске вне CWD.
- **Embedded Fallbacks**: Критически важные ассеты (шрифты, базовые текстуры кнопок) вшиваются в бинарный файл (`include_bytes!`) для обеспечения отказоустойчивости при отсутствии внешних файлов.

### 1.1 Asset Manifest Schema (RON)

Манифест служит единой точкой входа для всех ресурсов игры. Это позволяет подменять ассеты (рескиннинг) без изменения кода.

**Structure Examples:**

- `fonts`: Словарь путей к шрифтам (ключи: `main`, `bold`).
- `audio`: Привязка логических имен (напр., `click`, `music_menu`) к путям файлов.
- `branding`: Логотипы и фоны для Splash и Main Menu.

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
| 0.2.0   | 2026-02-19 | Agent  | Added Base Path and Fallbacks details |
| 0.3.0   | 2026-02-19 | Agent  | Added Asset Manifest Schema from assets.ron |
