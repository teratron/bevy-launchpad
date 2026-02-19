# UI & Experience Standards

**Version:** 0.1.0
**Status:** Draft

---

## Overview

Visual standards, UI hierarchy, and interaction principles for AAA-quality feel.

## Related Specifications

- [architecture.md](architecture.md) - Layered model (L2 Infrastructure).

## 1. UI Principles

- **Navigation Stack**: История переходов для работы кнопки "Back" и ESC.
- **Settings Commitment**: Явное применение изменений (Apply/Reset).

## 2. Detailed Design

### 2.1 Layering & Z-Order

- **World (0-99)**: Игровые объекты.
- **HUD (100-199)**: Интерфейс в игре.
- **Menus (200-299)**: Основные экраны.
- **Settings/Modals (300-499)**: Поверх всех меню.
- **Error/Debug (500+)**: Высший приоритет.

### 2.2 Transitions & Visual Polish

- **Effects**: Fade-in/Fade-out (затухание), Vignette, Blur. Плавная смена стейтов.
- **Button Juiciness**:
  - Анимация Hover: Масштабирование (1.05x) или изменение яркости.
  - Анимация Click: Визуальное "нажатие" (0.95x).
- **Audio Feedback**: Профессиональные звуковые эффекты (SFX) при наведении и клике на любой интерактивный объект.

### 2.3 Accessibility

- **High Contrast**: Режим повышенной контрастности.
- **Font Scaling**: Динамическое изменение размера шрифта.

---

## Document History

| Version | Date       | Author | Description   |
| :---    | :---       | :---   | :---          |
| 0.1.0   | 2026-02-19 | Agent  | Initial Draft |
