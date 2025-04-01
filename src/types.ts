import type { Options as SwcOptions } from '@swc/core';

export interface Options {
  /**
   * Base options for SWC.
   */
  baseOptions?: SwcOptions | ((code: string, id: string) => SwcOptions);
}
