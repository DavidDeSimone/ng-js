import { DB } from "https://deno.land/x/sqlite/mod.ts";

const db = new DB("test.db");
db.execute(`
  CREATE TABLE IF NOT EXISTS people (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT
  )
`);

// console.log('Hello js');
console.log("line #3");
lisp.bufferMenu()
.then((result) => console.log("Follow on... " + result))
.then(async () => {
    await lisp.getBufferCreate("newBuffer");
    await lisp.switchToBuffer("newBuffer");
    await lisp.setBuffer("newBuffer");
});
