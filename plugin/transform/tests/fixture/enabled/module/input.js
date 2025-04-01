import __mod from "esm";

dbg("hello, world");

function sum(a, b) {
  const result = dbg(a + b);
  dbg("sum res", result);
}

function inner() {
  var dbg = () => undefined;

  dbg(1, 2, 3);
}

sum(5, 10);
inner();
