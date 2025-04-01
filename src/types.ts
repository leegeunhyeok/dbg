import type { Options as SwcOptions } from '@swc/core';

export interface Options {
  baseOptions?: SwcOptions | ((code: string, id: string) => SwcOptions);
}
