const __mod = require('cjs');

dbg('hello, world');

function sum(a, b) {
  const result = dbg(a + b);
  dbg('sum res', result);
}

function inner() {
  var dbg = () => undefined;

  dbg(1, 2, 3);
}

const callback = (cb) => cb();
function nested() {
  callback(() => {
    callback(() => {
      callback(() => {
        dbg('in nested callback', { value: callback(() => dbg('value callback')) });
      });
    });
  });
}

sum(5, 10);
inner();
nested();
