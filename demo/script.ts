function sum(a: number, b: number) {
  const [res, _a, _b] = dbg(a + b, a, b);
  return res;
}

const result = dbg(sum(10, 5));

console.log('From console.log!:', result);
