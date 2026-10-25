Until now the "mod server" was a `sleep`. Next the downloader talks to a real one: it asks a mod site which files a modpack has and gets text back. Before the Rust, here is what actually travels over the wire, because the boss needs every part of it.

**Request and response.** The web works in pairs. The client sends a **request**: a **method**, `GET` for "give me" or `POST` for "take this", and a **path** such as `/mod/sodium.json` on the server's address. The server sends back a **response**: a status code, a few header lines, and a **body** with the content.

**Status codes.** The first line of every response has a three-digit number. `200` means OK. A `4xx` means the request was wrong: `404 Not Found`, nothing at that path. A `5xx` means the server failed: `500` or `503`, often for a moment. So a 5xx may be worth one retry, and a 404 never is. A 404 is still an *answer*, so the network itself worked.

```text
GET /mod/sodium.json HTTP/1.1
Host: mods.example.com

HTTP/1.1 200 OK
Content-Type: application/json

{"title": "Sodium", "files": [{"name": "sodium-0.6.jar", "size": 4200000}, {"name": "old.jar", "size": null}]}
```

**JSON.** The body above is **JSON**, the text format most servers answer in. It has a few shapes: an **object** in `{ }` holds named fields, `"name": value`; an **array** in `[ ]` is a list; then strings in double quotes, numbers, `true` / `false`, and `null` for "no value". Shapes nest, so an array of objects is a list of records, like a modpack manifest.

**Text in, values out.** For Rust the body is only text. Turning JSON text into Rust values is called **deserializing**, and turning values into JSON text is **serializing**. The next quests deserialize with two crates: `reqwest` sends the request, and `serde` turns the JSON into your own structs.

## Your turn

- Answer the questions. No code this time.
