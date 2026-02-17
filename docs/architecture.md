# Architecture

## Modules

- core/     → Логика фреймворка (boot, states, loading, splash)
- ui/       → Визуальные компоненты (menu, widgets, theme, transitions)
- locale/   → Локализация (пока заглушки)
- utils/    → Утилиты (single_instance отлично сделан!)

## Dependency graph

```plaintext
utils  ◄──────────────────────────────┐
  ▲                                   │
core  (boot, states, loading, splash) │
  ▲                                   │
ui    (theme, widgets, menu, modal)   │
  ▲                                   │
locale                                │
  ▲                                   │
lib.rs  (LaunchpadPlugin) ────────────┘
```

## Automatic transition flow

```plaintext
Booting ──(boot done)──► Loading ──(assets ready)──► Splash ──(screens done)──► Menu
                                                          │
                                        (skip_all=true or no screens)
                                                          ▼
                                                        Menu
```

## File structure

```plaintext
src/
├── lib.rs                        # Главная точка входа
├── prelude.rs                    # Удобные re-exports
│
├── core/                         # 🎯 ЯДРО ФРЕЙМВОРКА
│   ├── mod.rs                    # Core plugin и re-exports
│   ├── plugin.rs                 # LaunchpadCorePlugin
│   │
│   ├── boot/                     # Boot sequence
│   │   ├── mod.rs
│   │   ├── sequence.rs           # Инициализация
│   │   ├── paths.rs              # Определение путей
│   │   └── config.rs             # Конфигурация загрузки
│   │
│   ├── states/                   # State management
│   │   ├── mod.rs
│   │   ├── app_state.rs          # NEW: built-in AppState enum
│   │   ├── machine.rs            # Generic state machine
│   │   └── transitions.rs        # Переходы между состояниями
│   │
│   ├── loading/                  # Asset loading
│   │   ├── mod.rs
│   │   ├── tracker.rs            # Трекинг загрузки
│   │   ├── manifest.rs           # Манифест ассетов
│   │   └── progress.rs           # Прогресс загрузки
│   │
│   └── splash/                   # Splash screens (логика)
│       ├── mod.rs
│       ├── sequence.rs           # Последовательность экранов
│       └── timer.rs              # Таймер показа
│
├── ui/                           # 🎨 UI КОМПОНЕНТЫ
│   ├── mod.rs                    # UI plugin и re-exports
│   ├── plugin.rs                 # LaunchpadUiPlugin
│   │
│   ├── theme/                    # Система тем
│   │   ├── mod.rs
│   │   ├── colors.rs             # Цветовые схемы
│   │   ├── fonts.rs              # Шрифты
│   │   ├── spacing.rs            # Отступы и размеры
│   │   └── presets.rs            # dark/light/custom темы
│   │
│   ├── widgets/                  # Базовые UI виджеты
│   │   ├── mod.rs
│   │   ├── button.rs             # Кнопка
│   │   ├── slider.rs             # Слайдер
│   │   ├── dropdown.rs           # Выпадающий список
│   │   ├── checkbox.rs           # Чекбокс
│   │   └── input.rs              # Текстовое поле
│   │
│   ├── menu/                     # Системы меню
│   │   ├── mod.rs
│   │   ├── main_menu.rs          # Главное меню
│   │   ├── pause_menu.rs         # Пауза
│   │   │
│   │   └── settings/             # Настройки
│   │       ├── mod.rs
│   │       ├── panel.rs          # Основная панель
│   │       ├── graphics.rs       # Графика
│   │       ├── audio.rs          # Аудио
│   │       ├── controls.rs       # Управление
│   │       └── general.rs        # Общие настройки
│   │
│   ├── modal/                    # Модальные окна
│   │   ├── mod.rs
│   │   ├── dialog.rs             # Базовый диалог
│   │   ├── confirm.rs            # Подтверждение
│   │   └── alert.rs              # Уведомление
│   │
│   ├── transitions/              # Визуальные переходы
│   │   ├── mod.rs
│   │   ├── fade.rs               # Затемнение
│   │   ├── slide.rs              # Сдвиг
│   │   └── zoom.rs               # Зум
│   │
│   └── splash/                   # Splash screens (рендеринг)
│       ├── mod.rs
│       ├── renderer.rs           # Отрисовка splash
│       └── animations.rs         # Анимации
│
├── locale/                       # 🌍 ЛОКАЛИЗАЦИЯ
│   ├── mod.rs
│   ├── plugin.rs                 # LocalePlugin
│   ├── fluent.rs                 # Fluent integration
│   ├── language.rs               # Enum языков
│   └── utils.rs                  # Вспомогательные функции
│
└── utils/                        # 🛠️ УТИЛИТЫ
    ├── mod.rs
    ├── singleton.rs              # Single instance lock
    ├── validation.rs             # Валидация настроек
    ├── platform.rs               # Platform-specific код
    └── persistence.rs            # Сохранение/загрузка
```
