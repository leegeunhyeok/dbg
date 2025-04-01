const { _: __dbg } = require("unplugin-dbg/runtime");
const __mod = require("cjs");
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
function inner() {
    var dbg = ()=>undefined;
    dbg(1, 2, 3);
}
sum(5, 10);
inner();
