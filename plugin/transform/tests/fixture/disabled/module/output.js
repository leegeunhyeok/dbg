import { _ as __dbg } from "unplugin-dbg/runtime";
import __mod from "esm";
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
