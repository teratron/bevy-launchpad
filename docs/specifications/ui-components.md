# UI & Experience Standards

**Version:** 0.2.1
**Status:** Draft
**Roadmap Phase:** Phase 2

## Overview

General rules for building user interfaces, visual language, and interaction patterns for the Bevy Launchpad framework.

## Related Specifications

- [architecture.md](architecture.md) - Layered model and infrastructure context (§2).

## 1. UI Principles

- **Responsiveness**: All elements must scale correctly across resolutions.
- **Consistency**: Unified color palette and typography.
- **Micro-interactions**: Subtle animations for all user actions.
- **Navigation Stack**: История переходов для работы кнопки "Back" и ESC. Каждое подменю ОБЯЗАНО иметь кнопку "Назад" и поддержку закрытия по `Esc`.
- **Settings Commitment**: Явное применение изменений (Apply/Reset).

## 2. Layering (Z-Order)

| Layer | Priority | Usage |
| :--- | :--- | :--- |
| **Splash** | 1000 | Splash screens and logos. |
| **Overlay** | 500  | Modals, settings overlays, pause menu. |
| **HUD**     | 100  | In-game indicators. |
| **Background** | 0 | Root menu or game world. |

## 3. Transitions & Visual Polish

Все переходы между состояниями должны быть плавными.

- **Fade-in / Fade-out**: Рекомендуемое время перехода 200-500мс.
- **Splash Screens**: Скипаемые (после 1 сек) через любую клавишу или клик.
- **Progressive Loading**: Индикатор прогресса (%) с названиями групп загружаемых ассетов.
- **Button Juiciness**:
  - Hover: Увеличение масштаба (1.05x), смена цвета фона/бордера.
  - Click: Сжатие (0.95x), воспроизведение SFX.

## 4. Accessibility

- **High Contrast Mode**: Поддержка альтернативной цветовой схемы.
- **Font Scaling**: Возможность увеличения шрифтов для доступности.

## Document History

| Version | Date       | Author | Description   |
| :---    | :---       | :---   | :---          |
| 0.1.0   | 2026-02-19 | Agent  | Initial Draft |
| 0.2.0   | 2026-02-19 | Agent  | Added Nav Safety (ESC/Back) requirements |
| 0.2.1   | 2026-02-19 | Agent  | Added Roadmap Phase field + Fixed broken reference |
