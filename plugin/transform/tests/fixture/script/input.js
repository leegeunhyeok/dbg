const __mod = require("cjs");

dbg("hello, world");

function sum(a, b) {
  const result = dbg(a + b);
  dbg("sum res", result);
}

sum(5, 10);
