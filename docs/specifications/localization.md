# Localization Specification

**Version:** 0.2.1
**Status:** Draft
**Roadmap Phase:** Phase 2

## Overview

Система локализации обеспечивает поддержку множества языков, форматов дат и региональных настроек. Использование **Localization-First** принципа означает, что весь пользовательский текст должен проходить через систему перевода.

## Related Specifications

- [ui-components.md](ui-components.md) - Text rendering standards.
- [data-management.md](data-management.md) - Locale asset storage.

## 1. Core Principles

### 1.1 Project Fluent Integration

Использование формата **Fluent (.ftl)** для поддержки сложной грамматики (роды, числа).

- **Key-based access**: Доступ к строкам через уникальные идентификаторы.
- **Namespace splitting**: Разделение файлов локализации (menu.ftl, gameplay.ftl).

### 1.2 Regional Assets

- **Audio Locales**: Поддержка региональных аудиофайлов.
- **Asset Fallback**: Если локализованный ресурс (строка или звук) отсутствует, система автоматически откатывается к English (en-US).

## 2. Key Registry (Fluent)

- **File Path**: `assets/locales/{lang}/text/menu.ftl`
- **Standard Keys**:
  - `menu-title`: Заголовок игры
  - `menu-play`: Кнопка начала игры
  - `menu-settings`: Кнопка настроек
  - `menu-exit`: Кнопка выхода
  - `settings-title`: Заголовок настроек
  - `tab-graphics`, `tab-audio`: Названия вкладок настроек.

## Document History

|Version|Date|Author|Description|
|:---|:---|:---|:---|
|0.1.0|2026-02-19|Agent|Initial Draft|
|0.2.0|2026-02-19|Agent|Added Key Registry and Fallback details|
|0.2.1|2026-02-19|Agent|Added Roadmap Phase field|
