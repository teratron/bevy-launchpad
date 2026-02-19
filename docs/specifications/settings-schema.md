# Settings Data Schema (RON)

**Version:** 1.0.0
**Status:** Stable

---

## Overview

Техническая спецификация структуры файла `settings.ron`, используемого для хранения пользовательских предпочтений.

## Related Specifications

- [data-management.md](data-management.md) - Persistence layer and Version Guard.
- [settings-module.md](settings-module.md) - UI interaction logic.

## 1. Data Structure Analysis

Файл использует формат **RON** (Rusty Object Notation) для нативной интеграции с Bevy и поддержки типизированных перечислений (Enums).

### 1.1 Root Object

- `version`: (u32) Версия схемы для миграций.
- `general`: (Struct) Системные и UI настройки.
- `graphics`: (Struct) Параметры рендеринга и окна.
- `audio`: (Struct) Параметры звуковой подсистемы.
- `mouse`: (Struct) Параметры манипулятора.
- `controls`: (Struct) Маппинг клавиш.

## 2. Detailed Schema

### 2.1 Graphics Block

- `fullscreen`: (bool) Полноэкранный режим.
- `resolution`: (u32, u32) Кортеж ширины и высоты.
- `vsync`: (bool) Вертикальная синхронизация.
- `rendering`:
  - `quality`: (Enum: Low, Medium, High, Ultra).
  - `scale`: (f32: 0.5 - 2.0) Масштаб внутреннего рендеринга.

### 2.2 Audio Block

- `master_volume`: (f32: 0.0 - 1.0).
- `music_volume`: (f32: 0.0 - 1.0).
- `sfx_volume`: (f32: 0.0 - 1.0).
- `ui_volume`: (f32: 0.0 - 1.0).

### 2.3 Controls Block

- Разделен на подблоки: `movement`, `actions`, `menus`, `misc`, `debug`.
- Значения: (String) Названия клавиш Bevy (напр., "W", "Space", "LeftShift").

## 3. Storage Location

- **Template**: `assets/configs/settings.ron` (библиотечный шаблон).
- **Runtime User Path**:
  - **Windows**: `%APPDATA%/{Org}/{App}/settings.ron`
  - **Linux**: `~/.config/{app}/settings.ron`
  - **macOS**: `~/Library/Application Support/{org}.{app}/settings.ron`

---

## Document History

| Version | Date       | Author | Description   |
| :---    | :---       | :---   | :---          |
| 1.0.0   | 2026-02-19 | Agent  | Initial export from settings.toml |
