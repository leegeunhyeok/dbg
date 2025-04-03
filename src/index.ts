import * as module from 'module';
import * as swc from '@swc/core';
import { mergeWith } from 'es-toolkit';
import { createUnplugin, type UnpluginFactory } from 'unplugin';
import type { Options } from './types';
import { getBaseSwcParserConfig, mergeArray } from './utils';

const resolveModule = module.createRequire(
  typeof __filename !== 'undefined' ? __filename : import.meta.url
).resolve;

const pluginWasmPath = resolveModule('unplugin-dbg/swc-plugin');

export const unpluginFactory: UnpluginFactory<Options | undefined> = (
  options
) => ({
  name: 'unplugin-dbg',
  enforce: 'pre',
  transform(code, id) {
    const mergedOptions = mergeWith<swc.Options, swc.Options>(
      {
        filename: id,
        jsc: {
          parser: getBaseSwcParserConfig(id),
          experimental: {
            plugins: [[pluginWasmPath, { enabled: options?.enabled ?? true }]],
          },
        },
      },
      options?.baseSwcOptions
        ? typeof options.baseSwcOptions === 'function'
          ? options.baseSwcOptions(code, id)
          : options.baseSwcOptions
        : {},
      mergeArray
    );

    if (mergedOptions.jsc?.parser == null) {
      return;
    }

    return swc.transform(code, mergedOptions);
  },
});

export const unplugin = /* #__PURE__ */ createUnplugin(unpluginFactory);

export default unplugin;
