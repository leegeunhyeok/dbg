import { defineConfig } from 'tsup';

export default defineConfig({
  entry: ['src/*.ts', 'src/runtime/dbg.ts', 'src/runtime/dbg-shim.ts'],
  bundle: true,
  splitting: false,
  sourcemap: false,
  clean: true,
  dts: true,
  format: ['esm', 'cjs'],
});
