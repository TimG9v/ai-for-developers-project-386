# AGENTS.md

Календарь: бекенд Rust + axum (`backend/`), фронтенд Next.js 16 (App Router, TypeScript, Tailwind, shadcn/ui) (`frontend/`), CI — `.github/workflows/`.

## Команды

Запуск, тесты и линтеры — через Makefile в корне:

|       | backend                                                                                           | frontend                                       |
| ----- | ------------------------------------------------------------------------------------------------  | --------------------------------------------   |
| dev   | `make dev-backend` — `cargo run`, слушает `127.0.0.1:8081`                                        | `make dev-frontend` — `npm run dev`            |
| test  | `make test-backend` — `cargo test`                                                                | `make test-frontend` — `npm test` (vitest)     |
| lint  | `make lint-backend` — `cargo fmt --check` + `cargo clippy --all-targets -- -D warnings`           | `make lint-frontend` — `npm run lint` (ESLint) |

Оба приложения разом: `make dev`, `make test`, `make lint`.

## Структура

- `backend/` — crate `backend`: `src/lib.rs` (роутер `app()`), `src/main.rs` (bind), `tests/smoke.rs` (старт сервера, 200 на `GET /health`)
- `frontend/` — `app/` (страницы), `components/` (shadcn/ui), `tests/smoke.test.tsx`
- `.github/workflows/` — `backend-ci.yml` и `frontend-ci.yml` (тесты + линтеры на push и PR), `security.yml` (cargo audit / npm audit по lock-файлам), `release-please.yml` (release-PR после мержа в `main`)
- Пины версий: `.tool-versions` (rust), `.nvmrc` (node). Rust — в трёх местах: `.tool-versions` + `backend-ci.yml` + `security.yml`, менять все; node — только `.nvmrc` (CI читает файл напрямую).

## Правила

- Коммиты — Conventional Commits: `feat:`, `fix:`, `chore:`, `ci:`, `docs:` (scope допустим: `feat(backend):`). Формат касается и коммитов агента: release-please строит из истории коммитов changelog и semver-версию.
- Frontend — Next.js 16, отличается от привычной версии: перед правкой фронтенда читай гайд в `frontend/node_modules/next/dist/docs/` (блок правил автогенерируется `next dev` в `frontend/AGENTS.md`).
