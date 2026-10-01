# Календарь звонков


[![hexlet-check](https://github.com/TimG9v/ai-for-developers-project-386/actions/workflows/hexlet-check.yml/badge.svg)](https://github.com/TimG9v/ai-for-developers-project-386/actions)

Разработайте совместно с ИИ сервис для бронирования календаря

Учебный проект Хекслета: https://ru.hexlet.io/programs/ai-for-developers
Как это должно работать: https://files.hexlet.app/a/2ipc5m

## Стек

- Разное

## Установка

<!-- Опишите установку: клонирование, зависимости, переменные окружения -->

```bash
git clone https://github.com/TimG9v/ai-for-developers-project-386.git
cd ai-for-developers-project-386
```

Локальный запуск без Docker (нужны Rust и Node из `.tool-versions`/`.nvmrc`):

```bash
make dev   # backend на :8081, frontend на :3000
```

## Использование

Опубликованное приложение: **https://calendar-zvonok.onrender.com**

Страница записи — выбор типа встречи и подтверждение записи:

![Страница записи: типы встреч и подтверждение записи](docs/context/task-6/Screenshot%20From%202026-10-01%2014-37-10.png)

Страница владельца — создание типов, публикация слотов, предстоящие встречи:

![Страница владельца: формы создания типа и публикации слота, предстоящие встречи](docs/context/task-6/Screenshot%20From%202026-10-01%2014-37-41.png)

Тот же образ локально — приложение поднимает backend и frontend само и
отвечает на порту из переменной окружения `PORT`:

```bash
docker build -t calendar .
docker run -e PORT=8080 -p 8080:8080 calendar
```

После запуска: `http://127.0.0.1:8080` — главная, `/booking` — страница
записи, `/admin` — страница владельца, `/health` — статус. Данные
хранятся в памяти и обнуляются при перезапуске контейнера.

---

<details>
<summary>Автоматические тесты Хекслета</summary>

Тесты запускаются на каждый коммит. За запуск отвечает файл `.github/workflows/hexlet-check.yml` — не удаляйте и не переименовывайте ни его, ни репозиторий.

</details>

## О Хекслете

[Хекслет](https://ru.hexlet.io/) — школа программирования: авторские программы обучения с практикой, поддержкой наставников и реальными проектами, которые остаются в резюме. Этот репозиторий — один из таких проектов.
