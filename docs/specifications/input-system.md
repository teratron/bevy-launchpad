# Input System

**Version:** 0.1.0
**Status:** Draft

---

## Overview

Action-based input management system for Bevy Launchpad.

## Related Specifications

- [architecture.md](architecture.md) - Layered model context.

## 1. Design Principles

- **Action-based**: Код ссылается на действия (напр., `Jump`), а не на кнопки.
- **Agnostic**: Поддержка клавиатуры, мыши и геймпада через единый интерфейс.
- **Remapping**: Возможность динамической смены биндов игроком.

## 2. Detailed Design

Анонимная система "Action-based" ввода, работающая через маппинги.

| Feature | Description |
| :--- | :--- |
| **Input Remapping** | Поддержка клавиатуры/мыши/геймпада с возможностью смены биндов. |
| **Juiciness** | Сглаживание ввода (deadzones) для геймпадов. |

---

## Document History

| Version | Date       | Author | Description   |
| :---    | :---       | :---   | :---          |
| 0.1.0   | 2026-02-19 | Agent  | Initial Draft |
