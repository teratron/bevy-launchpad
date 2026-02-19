# Gameplay Configuration Pattern

**Version:** 0.2.1
**Status:** Draft
**Roadmap Phase:** Phase 3

---

## Overview

Спецификация механизма хранения и доступа к игровым константам (баланс, физика, правила мира). В отличие от `settings.ron`, эти параметры определяют логику игры, а не предпочтения пользователя.

## Related Specifications

- [architecture.md](architecture.md) - Layered model (L4 Game Logic).
- [data-management.md](data-management.md) - Asset loading and hot-reloading.

## 1. Motivation

AAA-проекты требуют выноса балансовых значений из кода во внешние файлы. Это позволяет:

- Менять параметры без пересборки проекта.
- Поддерживать "живой" тюнинг геймплея дизайнерами (Hot-Reloading).
- Упростить создание модификаций (Modding).

## 2. Detailed Design

### 2.1 Storage & Format

- **Format**: RON (Rusty Object Notation).
- **Location**: `assets/configs/gameplay.ron`.
- **Ownership**: Этот файл является частью игровых ресурсов (ReadOnly для игрока, но доступен для чтения движком).

### 2.2 ECS Integration

Библиотека предоставляет систему автоматической десериализации `gameplay.ron` в типизированный ресурс Bevy:

```
ECS Integration — pseudo-logic:

system move_player:
  input:  GameplayConfig resource (physics.gravitational_constant)
  input:  Query<Velocity> — all entities with velocity component
  action: apply gravitational_constant to each entity's vertical velocity
```

### 2.3 Hot-Reloading

При изменении `gameplay.ron` во время работы (в режиме `Dev`), движок должен генерировать `GameplayConfigChanged` событие, позволяя системам мгновенно адаптироваться к новым значениям баланса.

---

## Document History

| Version | Date       | Author | Description   |
| :---    | :---       | :---   | :---          |
| 0.1.0 | 2026-02-19 | Agent | Initial Draft |
| 0.2.0 | 2026-02-19 | Agent | Replaced Rust code with pseudo-logic (RULES §5) |
| 0.2.1 | 2026-02-19 | Agent | Added Roadmap Phase field |
