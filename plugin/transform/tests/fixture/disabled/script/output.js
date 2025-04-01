const { _: __dbg } = require("unplugin-dbg/runtime");
const __mod = require("cjs");
__dbg.shim("hello, world");
function sum(a, b) {
    const result = __dbg.shim(a + b);
    __dbg.shim("sum res", result);
}
function inner() {
    var dbg = ()=>undefined;
    dbg(1, 2, 3);
}
sum(5, 10);
inner();
