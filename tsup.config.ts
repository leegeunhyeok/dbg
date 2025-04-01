import { defineConfig } from 'tsup';

export default defineConfig({
  entry: ['src/plugin.ts', 'src/runtime.ts'],
  splitting: false,
  sourcemap: false,
  clean: true,
  dts: true,
  format: ['esm', 'cjs'],
});
