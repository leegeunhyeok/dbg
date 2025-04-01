import { _ as __dbg } from "unplugin-dbg/runtime";
import __mod from "esm";
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
