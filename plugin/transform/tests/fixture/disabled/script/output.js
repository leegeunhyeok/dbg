const { _: __dbg } = require("unplugin-dbg/runtime-shim");
const __mod = require('cjs');
__dbg('hello, world');
function sum(a, b) {
    const result = __dbg(a + b);
    __dbg('sum res', result);
}
function inner() {
    var dbg = ()=>undefined;
    dbg(1, 2, 3);
}
const callback = (cb)=>cb();
function nested() {
    callback(()=>{
        callback(()=>{
            callback(()=>{
                __dbg('in nested callback');
            });
        });
    });
}
sum(5, 10);
inner();
nested();
