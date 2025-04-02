const { _: __dbg } = require("unplugin-dbg/runtime");
const __mod = require('cjs');
__dbg.call(null, {
    expr: "'hello, world'",
    value: 'hello, world'
});
function sum(a, b) {
    const result = __dbg.call(null, {
        expr: "a + b",
        value: a + b
    });
    __dbg.call(null, {
        expr: "'sum res'",
        value: 'sum res'
    }, {
        expr: "result",
        value: result
    });
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
                __dbg.call(null, {
                    expr: "'in nested callback'",
                    value: 'in nested callback'
                }, {
                    expr: "{\n  value: callback(()=>__dbg.call(null, {\n      expr: \"'value callback'\",\n      value: 'value callback'\n    }))\n}",
                    value: {
                        value: callback(()=>__dbg.call(null, {
                                expr: "'value callback'",
                                value: 'value callback'
                            }))
                    }
                });
            });
        });
    });
}
sum(5, 10);
inner();
nested();
