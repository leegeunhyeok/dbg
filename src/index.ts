import * as module from 'module';
import * as swc from '@swc/core';
import { mergeWith } from 'es-toolkit';
import { createUnplugin, type UnpluginFactory } from 'unplugin';
import type { Options } from './types';

const __require =
  typeof require === 'undefined'
    ? module.createRequire(import.meta.url)
    : require;

const pluginPath = __require.resolve('unplugin-dbg/swc-plugin');

function mergeArray(objValue: any[], srcValue: any[]) {
  if (Array.isArray(objValue)) {
    return objValue.concat(srcValue);
  }
}

export const unpluginFactory: UnpluginFactory<Options | undefined> = (
  options
) => ({
  name: 'unplugin-dbg',
  transform(code, id) {
    const mergedOptions = mergeWith<swc.Options, swc.Options>(
      {
        filename: id,
        jsc: {
          experimental: {
            plugins: [[pluginPath, { enabled: options?.enabled ?? true }]],
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

    return swc.transform(code, mergedOptions);
  },
});

export const unplugin = /* #__PURE__ */ createUnplugin(unpluginFactory);

export default unplugin;
