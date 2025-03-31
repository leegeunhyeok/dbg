import path from 'path';
import url from 'url';
import * as swc from '@swc/core';

const __dirname = url.fileURLToPath(import.meta.url);

const result = await swc.transform(
  `
function sum(a, b) {
  const result = dbg(a + b);
  dbg('sum res', result);
}

sum(5, 10);
`.trim(),
  {
    filename: './to/test.ts',
    jsc: {
      target: 'es2020',
      parser: {
        syntax: 'typescript',
      },
      experimental: {
        plugins: [
          [
            path.resolve(
              __dirname,
              '../../plugin/target/wasm32-wasip1/release/swc_plugin_dbg.wasm'
            ),
            {},
          ],
        ],
      },
    },
  }
);

console.log(result.code);
