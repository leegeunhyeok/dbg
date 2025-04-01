function sum(a, b) {
  const result = dbg(a + b);
  dbg('sum res', result, { a, b });
  return result;
}

const sumResult = dbg(sum(5, 10));
console.log('sumResult:', sumResult);
