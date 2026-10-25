The mod list lives in the mod manager's database, but a launcher, a web page or a friend's script wants it too. They have to ask, and the manager has to answer, even when they run on different machines. The rules for that conversation are **HTTP**, the same rules a browser uses for every web page.

**Ask and answer.** The program that asks is the **client** (a launcher, a browser, a script), and the program that answers is the **server** (the mod manager). The client sends one **request**; the server sends back one **response**. Then they are done, and the next question is a new request.

**What a request says.** A request has a **method**, the kind of thing you want: `GET` to read, `POST` to add, `PUT` to replace, `DELETE` to remove. It has a **path**, which thing: `/mods` for all mods, `/mods/7` for mod 7. A `POST` or `PUT` also carries a **body**, usually JSON, with the data being sent.

**What comes back.** The response starts with a **status code**, a three-digit number saying how it went, then its own body. `2xx` means it worked: `200 OK`, `201 Created` (something new was made), `204 No Content` (done, nothing to show). `4xx` means the client asked wrongly. `5xx` means the server broke.

```text
GET /mods/7               ->  200 OK         {"id": 7, "name": "sodium"}
POST /mods {"name":""}    ->  400 Bad Request  {"error": "name is empty"}
GET /mods/99              ->  404 Not Found
DELETE /status            ->  405 Method Not Allowed
GET /mods (db is down)    ->  500 Internal Server Error
```

**The codes you'll use.** `400 Bad Request`: the body makes no sense. `404 Not Found`: no such thing at that path. `405 Method Not Allowed`: the path exists but not with that method. `500 Internal Server Error`: the server's own fault, never the client's. Picking the right one lets the launcher show the right message.

**Routes.** On the server, a **route** is a method and a path matched to the function that answers them: "`GET /mods/{id}` is answered by `get_mod`". The `{id}` part is a gap that any value fits. An **API** is just the list of routes a server offers.

**Where it runs.** In a real app the server listens on a **port**, a numbered door on the machine (`localhost:3000` is port 3000 on your own computer; a Minecraft server uses 25565). The tests in this world skip the network entirely and hand requests straight to the server code, so they run in milliseconds.

## Your turn

Answer the three questions.
