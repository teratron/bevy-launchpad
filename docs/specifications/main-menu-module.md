# Main Menu Module

**Version:** 0.1.0
**Status:** Draft

---

## Overview

Главное меню — это основной интерактивный интерфейс игры. Оно обеспечивает доступ к игровому процессу, настройкам конфигурации и выходу из приложения.

## Related Specifications

- [ui-components.md](ui-components.md) - Visual standards and polish.
- [localization.md](localization.md) - Localization principles.
- [architecture.md](architecture.md) - State management context.

## 1. Functional Requirements

### 1.1 Root Screen

- **Logo/Title**: Центральное отображение визуального стиля игры.
- **Actions Bar**: Список основных команд.
  - **Play**: Запуск игрового цикла (переход в `Loading` -> `InGame`).
  - **Settings**: Открытие окна конфигурации.
  - **Credits**: Экран авторов.
  - **Exit**: Выход из приложения (с подтверждением).

### 1.2 UX Standards

- **Keyboard/Gamepad Navigation**: Полная поддержка выбора элементов без мыши.
- **Back Action**: Клавиша ESC или кнопка "Назад" всегда возвращает на предыдущий уровень в Navigation Stack.

---

## 2. Detailed Design

### 2.1 State Flow

```mermaid
graph TD
    Splash[Splash Screen] --> MainMenu[Main Menu: Root]
    MainMenu --> PlaySequence[Play: Slot Selection / Loading]
    MainMenu --> SettingsWindow[Settings Window]
    MainMenu --> CreditsScreen[Credits Screen]
    MainMenu --> ExitConfirm[Exit Confirmation Modal]
```

### 2.2 Integration Points

- **Events**: Генерация `StartGameEvent` при нажатии Play.
- **Resources**: Использование `MenuTheme` для стилизации элементов.

---

## Document History

| Version | Date       | Author | Description   |
| :---    | :---       | :---   | :---          |
| 0.1.0   | 2026-02-19 | Agent  | Initial Draft |
