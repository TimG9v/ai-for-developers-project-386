# step-5-release-please-fix

- Дата: 2026-09-29 12:48
- Ветка: main
- Шаг: правка шага 5 — фиксы WARNING №1–2 из ревью

## Причина

Ревью реализации шага 5: спека (и реализация) опирались на устаревшую
официальную документацию release-please-action.

1. `google-github-actions/release-please-action` — репозиторий
   депрекирован, разработка переехала в `googleapis/release-please-action`
   (real-run: README обоих репозиториев, webfetch 2026-09-29).
2. В permissions отсутствовало `issues: write`, которое предписывает
   актуальный README (лейблы `autorelease` на release-PR — Issues API).

## Изменение

- `.github/workflows/release-please.yml`:
  - `uses: google-github-actions/...` → `uses: googleapis/release-please-action@v4`
    (интерфейс `@v4` не изменился — примеры нового README);
  - добавлено `issues: write` + комментарии-обоснования к обеим правкам.

## Проверка (real-run, 2026-09-29)

| # | Команда                                        | Результат                |
| - | ---------------------------------------------- | ------------------------ |
| 1 | actionlint 1.7.12 по release-please.yml        | exit 0, 0 errors         |
| 2 | YAML-проверка дублей ключей (python)           | OK                       |
| 3 | `act -l`                                       | workflow виден, push     |

Существование `googleapis/release-please-action` и его `@v4`
подтверждено официальным README (webfetch), а не памятью модели.
Реальный прогон release-please остаётся заблокирован отсутствием
GitHub-remote (блокер спеки шага 5).

## Коммит (делает пользователь)

`chore: add release-please workflow` (без изменений)
