const { dbg: __dbg } = require("unplugin-dbg/runtime");
const __mod = __dbg.call(null, {
    expr: '"cjs"',
    value: "cjs"
});
__dbg.call(null, {
    expr: '"hello, world"',
    value: "hello, world"
});
function sum(a, b) {
    const result = __dbg.call(null, {
        expr: "a + b",
        value: a + b
    });
    __dbg.call(null, {
        expr: '"sum res"',
        value: "sum res"
    }, {
        expr: "result",
        value: result
    });
}
sum(5, 10);
