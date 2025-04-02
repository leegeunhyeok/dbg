import fs from 'fs';
import path from 'path';
import url from 'url';
import * as esbuild from 'esbuild';
import plugin from '../src/esbuild';

const __dirname = url.fileURLToPath(path.dirname(import.meta.url));
const projectRoot = path.resolve(__dirname, '..');
const scriptPath = path.resolve(__dirname, 'script.ts');

const dbgResolvePlugin: esbuild.Plugin = {
  name: 'dbg-resolve',
  setup(build) {
    build.onResolve({ filter: /^unplugin-dbg\/runtime$/ }, () => ({
      path: '@dbg/runtime',
      namespace: 'dbg',
    }));

    build.onResolve({ filter: /^unplugin-dbg\/runtime-shim$/ }, () => ({
      path: '@dbg/runtime-shim',
      namespace: 'dbg-shim',
    }));

    build.onLoad({ filter: /.*/, namespace: 'dbg' }, async () => ({
      loader: 'js',
      contents: await fs.readFileSync(
        path.join(projectRoot, 'dist/runtime/dbg.js'),
        'utf-8'
      ),
    }));

    build.onLoad({ filter: /.*/, namespace: 'dbg' }, async () => ({
      loader: 'js',
      contents: await fs.readFileSync(
        path.join(projectRoot, 'dist/runtime/dbg-shim.js'),
        'utf-8'
      ),
    }));
  },
};

const buildResult = await esbuild.build({
  bundle: true,
  entryPoints: [scriptPath],
  write: false,
  plugins: [
    dbgResolvePlugin,
    plugin({
      baseSwcOptions: (_, id) => ({
        filename: path.basename(id),
      }),
    }),
  ],
});

// eslint-disable-next-line no-eval
eval(Buffer.from(buildResult.outputFiles[0].contents).toString());
