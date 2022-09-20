import { DB } from "https://deno.land/x/sqlite/mod.ts";

const db = new DB("test.db");
db.execute(`
  CREATE TABLE IF NOT EXISTS people (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT
  )
`);

await lisp.bufferMenu();
await lisp.getBufferCreate("newBuffer");
await lisp.switchToBuffer("newBuffer");
await lisp.setBuffer("newBuffer");
