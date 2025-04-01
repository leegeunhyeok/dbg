import fs from 'fs';
import path from 'path';
import url from 'url';
import * as esbuild from 'esbuild';
import * as swc from '@swc/core';

const __dirname = url.fileURLToPath(path.dirname(import.meta.url));
const projectRoot = path.resolve(__dirname, '..');
const scriptPath = path.resolve(__dirname, 'script.js');
const script = await fs.promises.readFile(scriptPath, 'utf-8');

const result = await swc.transform(script, {
  filename: path.basename(scriptPath),
  jsc: {
    target: 'es2020',
    parser: {
      syntax: 'typescript',
    },
    experimental: {
      plugins: [
        [
          path.resolve(
            projectRoot,
            'plugin/target/wasm32-wasip1/release/swc_plugin_dbg.wasm'
          ),
          {},
        ],
      ],
    },
  },
});

const buildResult = await esbuild.build({
  bundle: true,
  stdin: {
    contents: result.code,
  },
  write: false,
  plugins: [
    {
      name: 'dbg-resolve',
      setup(build) {
        const resolveFlag = 'unplugin-dbg';

        build.onResolve({ filter: /^unplugin-dbg\/runtime$/ }, () => ({
          path: '@dbg/runtime',
          namespace: resolveFlag,
        }));

        build.onLoad({ filter: /.*/, namespace: resolveFlag }, async () => ({
          loader: 'js',
          contents: await fs.readFileSync(
            path.join(projectRoot, 'dist/runtime.js'),
            'utf-8'
          ),
        }));
      },
    },
  ],
});

// eslint-disable-next-line no-eval
eval(Buffer.from(buildResult.outputFiles[0].contents).toString());
