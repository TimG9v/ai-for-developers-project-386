import { client } from "@/src/client/client.gen";

// Вызовы SDK идут из серверных компонентов, поэтому адрес бекенда нужен
// на сервере: переменная BACKEND_URL, по умолчанию — dev-адрес из Makefile.
client.setConfig({
  baseUrl: process.env.BACKEND_URL ?? "http://127.0.0.1:8081",
});
