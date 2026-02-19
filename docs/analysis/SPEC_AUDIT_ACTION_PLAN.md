# Spec Audit — Action Plan

**Date:** 2026-02-19
**Project:** Bevy Launchpad
**Audited by:** Agent (Post-Init Review)
**Status:** Pending

---

## Overview

Первый аудит спецификаций после инициализации системы выявил 2 критических нарушения,
4 важных системных пропуска и 3 незначительных несоответствия.
Все изменения сгруппированы по файлам и приоритету. Применять в указанном порядке.

---

## Приоритет 🔴 — Критично

### 1. `gameplay-config.md` — удалить Rust-код

**Нарушение:** RULES.md §5, Workflow Guideline №1 — "No Code in Specs".

**Что сделать:**
Удалить блок с реальным Rust-кодом в секции `2.2 ECS Integration`:

```rust
// УДАЛИТЬ — этот блок:
fn move_player(
    config: Res<GameplayConfig>,
    mut query: Query<&mut Velocity>,
) {
    let gravity = config.physics.gravitational_constant;
}
```

Заменить на описание логики в виде псевдокода или prose:

```
ECS Integration — pseudo-logic:

system move_player:
  input:  GameplayConfig resource (physics.gravitational_constant)
  input:  Query<Velocity> — all entities with velocity component
  action: apply gravitational_constant to each entity's vertical velocity
```

**Версия после правки:** `0.1.0 → 0.2.0` (minor — содержимое секции изменено).
**Document History:** добавить строку `0.2.0 | 2026-02-19 | Agent | Replaced Rust code with pseudo-logic (RULES §5)`.

---

### 2. `api.md` — исправить статус и Document History

**Нарушение:** RULES.md §2 — статус изменён в обход lifecycle `Draft → RFC → Stable`.
Document History содержит только `0.1.0 Initial Draft`, статус `Stable` появился без RFC-фазы.

**Что сделать — вариант A (рекомендуется):**
Вернуть статус в `RFC`, добавить запись в историю:

```markdown
**Status:** RFC
```

```
| 0.2.0 | 2026-02-19 | Agent | Promoted to RFC for review (lifecycle fix) |
```

После явного одобрения — перевести в `Stable` с записью `0.3.0`.

**Что сделать — вариант B (если спека уже фактически одобрена):**
Оставить `Stable`, но добавить явные записи о пропущенных фазах:

```
| 0.2.0 | 2026-02-19 | Agent | RFC phase (retroactive) — no open questions |
| 1.0.0 | 2026-02-19 | Agent | Promoted to Stable — approved             |
```

> ⚠️ Требует решения владельца: выбрать вариант A или B.

---

## Приоритет 🟡 — Важно

### 3. Все спек-файлы — добавить поле `Roadmap Phase`

**Проблема:** Поле `Roadmap Phase` определено в шаблоне спеки, но отсутствует
во всех существующих файлах. Связь между спеками и ROADMAP существует только
в одну сторону.

**Что сделать:** В заголовок каждого файла добавить поле после `Status`:

```markdown
**Roadmap Phase:** {Phase 1 | Phase 2 | Phase 3 | Backlog}
```

**Маппинг по файлам** (согласно текущему ROADMAP.md):

| Файл | Roadmap Phase |
| :--- | :--- |
| `architecture.md` | Phase 1 |
| `api.md` | Phase 1 |
| `data-management.md` | Phase 1 |
| `input-system.md` | Phase 1 |
| `settings-schema.md` | Phase 1 |
| `main-menu.md` | Phase 2 |
| `settings-ui.md` | Phase 2 |
| `localization.md` | Phase 2 |
| `ui-components.md` | Phase 2 |
| `gameplay-config.md` | Phase 3 |

**Версия после правки:** `patch` для каждого файла (структурный хедер, не контент).
**Document History:** добавить строку `X.X.1 | 2026-02-19 | Agent | Added Roadmap Phase field`.

---

### 4. `ROADMAP.md` — три item без спек-файлов в Phase 3

**Проблема:** `Transition Orchestrator`, `Theme Engine`, `Accessibility` числятся
в Phase 3, но не имеют соответствующих `.md` файлов в `docs/specifications/`.
Это "сироты" — их нельзя отследить, проревьювить или обновить через workflow.

**Что сделать — вариант A (рекомендуется):**
Перенести все три item в `Backlog` с явной пометкой что спека ещё не создана:

```markdown
## Backlog (Unprioritized)

- **Transition Orchestrator** *(no spec yet)*: Синхронизация загрузки ассетов с визуальными переходами.
- **Theme Engine** *(no spec yet)*: Гибкая стилизация UI через RON-конфигурацию.
- **Accessibility** *(no spec yet)*: Высокий контраст, масштабирование шрифтов, base screen reader.
```

**Что сделать — вариант B:**
Создать три Draft-заготовки (`transition-orchestrator.md`, `theme-engine.md`, `accessibility.md`)
с минимальным наполнением (Overview + Motivation) и зарегистрировать в INDEX.md.

> ⚠️ Требует решения владельца: вариант A или B.

**Версия ROADMAP после правки:** `1.1.0 → 1.2.0`.

---

### 5. `ui-components.md` — исправить битую ссылку

**Проблема:** В `Related Specifications` указано:

```
- [architecture.md](architecture.md) - Section 6: UI Layer.
```

В `architecture.md` нет Section 6. Файл содержит секции 1–4 (Vision, Architecture, Detailed Design, AAA Quality Standards).

**Что сделать:** Исправить ссылку на актуальную — ближайший раздел это §2 Layered Model:

```markdown
- [architecture.md](architecture.md) - Layered model and infrastructure context (§2).
```

**Версия после правки:** `0.2.0 → 0.2.1` (patch — исправление ссылки).
**Document History:** `0.2.1 | 2026-02-19 | Agent | Fixed broken reference to architecture.md`.

---

## Приоритет 🟢 — Незначительно

### 6. `INDEX.md` — обновить под новую структуру

**Проблема:** Секция называется "Core Planning Files" (старый шаблон),
RULES.md не указан в списке системных файлов.

**Что сделать:** Переименовать секцию и добавить RULES.md:

```markdown
## System Files

- [ROADMAP.md](ROADMAP.md) - Live priority and phase tracker.
- [RULES.md](RULES.md) - Project constitution and standing conventions.
```

**Версия INDEX после правки:** `1.0.0 → 1.1.0`.

---

### 7. `RULES.md §5` — уточнить обязательные секции

**Проблема:** Текущая формулировка:
> "Every spec must have an Overview, Motivation, and Related Specifications section."

`Related Specifications` — не обязательная секция (не все спеки имеют зависимости).
Должна быть в списке обязательных только `Document History`.

**Что сделать:** Исправить §5:

```markdown
## 5. Content Rules

- No implementation code in spec files (no Rust, JS, Python, SQL, etc.).
- Pseudo-code and logic flows are permitted where necessary.
- Every spec must have: Overview, Motivation, and Document History sections.
- Related Specifications is required only if the spec depends on another spec.
```

**Версия RULES после правки:** `1.0.0 → 1.0.1` (patch — уточнение формулировки).

---

### 8. `RULES.md §7` — уточнить Language Convention

**Проблема:** Текущая формулировка:
> "English/Russian mixed mode (Russian for content, English for structure/headers is acceptable as per current state)."

Агент не может применять это правило последовательно — нет чёткого критерия.

**Что сделать:** Заменить на конкретное правило:

```markdown
- **Language**: Spec structure (headers, field names, status values) is always in **English**.
  Content sections (descriptions, design rationale) may be written in **Russian**.
  Both languages may coexist within the same file.
```

**Версия RULES после правки:** `1.0.1 → 1.1.0` (minor — правило стало применяемым).

---

## Предложение в RULES.md §7 — T2 триггер

**Паттерн обнаружен:** `architecture.md`, `data-management.md`, `settings-schema.md`
независимо описывают RON-формат и `current_exe()`-путь без единого стандарта.

**Предлагаемое правило для `Project Conventions`:**

```markdown
- **Config Format**: RON (Rusty Object Notation) is the standard format
  for all configuration files in this project (`settings.ron`, `gameplay.ron`, `assets.ron`).
- **Asset Path Resolution**: Asset directory is always resolved at runtime
  via the executable's location (`current_exe()`), not the working directory.
  This must be documented in any spec that references file paths.
```

> Добавить в RULES.md §7? (yes / no)

---

## Сводная таблица изменений

| # | Файл | Тип правки | Приоритет | Новая версия |
| :- | :--- | :--- | :--- | :--- |
| 1 | `gameplay-config.md` | Удалить Rust-код → псевдокод | 🔴 Критично | `0.1.0 → 0.2.0` |
| 2 | `api.md` | Исправить статус и Document History | 🔴 Критично | `1.0.0 → 0.2.0 / 1.0.0` |
| 3 | Все 10 спек | Добавить поле `Roadmap Phase` | 🟡 Важно | `+0.0.1` каждая |
| 4 | `ROADMAP.md` | Разрешить сироты Phase 3 | 🟡 Важно | `1.1.0 → 1.2.0` |
| 5 | `ui-components.md` | Исправить битую ссылку | 🟡 Важно | `0.2.0 → 0.2.1` |
| 6 | `INDEX.md` | Обновить структуру + RULES.md | 🟢 Незначительно | `1.0.0 → 1.1.0` |
| 7 | `RULES.md §5` | Уточнить обязательные секции | 🟢 Незначительно | `1.0.0 → 1.0.1` |
| 8 | `RULES.md §7` | Уточнить Language Convention | 🟢 Незначительно | `1.0.1 → 1.1.0` |
| 9 | `RULES.md §7` | Добавить RON + path conventions (T2) | 🟢 Предложение | `1.1.0 → 1.2.0` |

---

## Решения, требующие ответа владельца

Перед применением изменений необходимо получить ответ по двум пунктам:

1. **`api.md` статус** — вариант A (вернуть в RFC) или вариант B (ретроактивные записи)?
2. **Phase 3 сироты** — вариант A (перенести в Backlog) или вариант B (создать Draft-заготовки)?
3. **T2 правило** — добавить RON + path convention в RULES.md §7?

---

*Документ сгенерирован автоматически по результатам Post-Init Audit.*
*После получения ответов — применить изменения в порядке приоритета.*
