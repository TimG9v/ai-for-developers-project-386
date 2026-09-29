# step-4-ci-workflow-report

- Дата: 2026-09-29 12:04
- Ветка: main
- Шаг: шаг 4/6 — GitHub Actions, отчёт о проделанной работе

Задача: `20260929070259-step-4-ci-workflow.md`. Инструкцией пользователя
изменён layout: вместо единого `ci.yml` — три workflow по примеру проекта
AiForge (`security.yml`, `backend-ci.yml`, `frontend-ci.yml`).

## Что изменено

- `.github/workflows/backend-ci.yml` — **новый**. Job `check`
  (working-directory: backend): `checkout@v5`; `dtolnay/rust-toolchain@1.98.1`
  + `components: clippy, rustfmt`; `actions/cache@v5` (registry, git,
  `backend/target`; ключ = `hashFiles('backend/Cargo.lock')` + output
  `cachekey` экшена); `cargo fmt --check` →
  `cargo clippy --all-targets -- -D warnings` → `cargo test`. Триггеры push/PR,
  `paths`: `backend/**`, `.tool-versions`, сам workflow.
- `.github/workflows/frontend-ci.yml` — **новый**. Job `check`
  (working-directory: frontend): `checkout@v5`; `setup-node@v5` с
  `node-version-file: .nvmrc` + npm-кэш
  (`cache-dependency-path: frontend/package-lock.json`); `npm ci` →
  `npm run lint` → `npm test` → `npm run build`. `paths`: `frontend/**`,
  `.nvmrc`, сам workflow.
- `.github/workflows/security.yml` — **новый**. Триггеры push
  `[master, main]` + PR + `schedule: cron "0 6 * * *"`, `paths` — манифесты
  и lock-файлы. Jobs: `cargo-audit` (working-directory: backend;
  `dtolnay/rust-toolchain@1.98.1`; `cargo install cargo-audit --locked`;
  `cargo audit`) и `npm-audit` (working-directory: frontend; `setup-node@v5`
  с `node-version-file: .nvmrc`; `npm audit --audit-level=high --omit=dev`).
- `.github/workflows/ci.yml` — **заменён**: перемещён в `tmp/trash/ci.yml`
  (удаление — за пользователем; `tmp*` уже в `.gitignore`).

Несвязанное изменение в рабочем дереве: `M .gitignore` (`+tmp*`) —
происхождение не установлено (не вносилось в видимой части сессии,
mtime 11:37); не тронуто — пользователю решать, включать в коммит
или откатывать.

## Ключевые решения (почему именно так)

- **Выбор экшенов** — по примеру AiForge: `checkout@v5`, `setup-node@v5`,
  `dtolnay/rust-toolchain`, `actions/cache@v5` (вместо `@v4` /
  `Swatinem/rust-cache` из исходной спеки). Версии тулчейнов зеркалят
  `.tool-versions` (1.98.1) и `.nvmrc` (25.2.0).
- **`components: clippy, rustfmt`** — `dtolnay/rust-toolchain` ставит
  тулчейн с `--profile minimal`; без компонентов rustfmt/clippy отсутствуют
  (code-reading action.yml).
- **cargo-audit: явная установка тулчейна** — в примере rust берётся из
  предустановки раннера; явный `dtolnay/rust-toolchain@1.98.1` делает job
  самодостаточным (закреплённая версия) и проходимым в локальном `act`
  (без этого: `cargo: command not found` в medium-образе, run 4).
- **`paths`-фильтры** — по примеру; сам workflow входит в push-список, но не
  в PR-список (паттерн примера).
- **`npm run build` во frontend** — по примеру; скрипт существует,
  `next build` зелёный. prettier/type-check не добавлены — их нет в проекте,
  новые зависимости вне задачи.
- **Имя job `check`** — по примеру.

## Что проверено

### real-run

- `actionlint` 1.7.12 по всем трём файлам: **0 errors, exit 0**
  (правила shellcheck/pyflakes отключены — бинарники не установлены
  и установить не удалось).
- `act push` (локальная симуляция GitHub Actions на docker, образ
  `catthehacker/ubuntu:act-latest`), run 4: Backend ✅, Frontend ✅,
  npm-audit ✅, cargo-audit ❌ (`cargo` отсутствует в образе).
- Фикс cargo-audit (явный тулчейн) + повторный `actionlint`: 0 errors, exit 0.
- `act push` run 5: **exit 0 — все 4 job succeeded**:
  - Backend CI/check: fmt, clippy, test (1 тест ok), кэш сохранён
    (ключ `Linux-cargo-6a1ccac…-2026090148a2`);
  - Frontend CI/check: `npm ci`, lint, test, `next build` ✓;
  - Security/cargo-audit: cargo-audit 0.22.2, 1273 advisory, 62 зависимости,
    уязвимостей нет;
  - Security/npm-audit: `found 0 vulnerabilities`.
- Локально (первая половина сессии): `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test`, `npm ci`,
  `npm run lint`, `npm test`, `npm run build` — все exit 0.

Ограничение: act — не GitHub-раннер (ubuntu-24.04); реальные кэш-сервис,
`hashFiles` и предустановка раннера здесь не участвуют.

### code-reading

- `dtolnay/rust-toolchain` (action.yml): `--profile minimal`, вход
  `components`, output `cachekey` (hash rustc).
- `actions/setup-node@v5.0.0` (src/main.ts): `node-version-file` резолвится
  относительно `GITHUB_WORKSPACE` (корень репо) → `.nvmrc` на корне работает;
  входы `cache` / `cache-dependency-path` на месте.
- act v0.2.89 (`cmd/root.go`, case "Medium"): medium-образ =
  `catthehacker/ubuntu:act-latest` (текст промпта совпал с логом — тот же
  код).

Локальная предпроверка `cargo audit` не состоялась (asdf-shim не смог
резолвить `cargo-audit`; exit 0 в пайплайне был статус `tail`) — аудит
выполнен внутри act run 5, что сильнее.

## Что не проверено и почему

- **Реальный прогон GitHub Actions** — нет GitHub-remote (блокер №1 задачи).
  `act push` — ближайшая локальная замена.
- `shellcheck` по `run`-командам — бинарник недоступен (команды однострочные).

## Следующие шаги (пользователь)

1. Удалить trash: `rm tmp/trash/ci.yml` (или оставить — `tmp*` в `.gitignore`).
2. Создать репозиторий на GitHub, `git remote add origin …`,
   `git push -u origin main`.
3. Вкладка Actions: три workflow — **Backend CI**, **Frontend CI**,
   **Security** — должны быть зелёные (cargo-audit на GitHub займёт ещё
   ~2–4 мин на компиляцию cargo-audit).
4. Коммит: `ci: run tests and linters on push` (агент не коммитит).

## Источники и трейсы

- Логи act: `/tmp/opencode/act-run4.log`, `/tmp/opencode/act-run5.log`
  (эфирные пути).
- Бинарники: `/tmp/opencode/actionlint` (1.7.12), `/tmp/opencode/act`
  (v0.2.89) — эфирные пути.
- `~/.config/act/actrc`: `-P ubuntu-latest=catthehacker/ubuntu:act-latest`.
- Пример: `/home/timur/projects/AiForge/.github/workflows/` —
  `security.yml`, `backend-ci.yml`, `frontend-ci.yml`.
