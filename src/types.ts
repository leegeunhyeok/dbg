import type { Options as SwcOptions } from '@swc/core';

export interface Options {
  /**
   * Whether to enable the plugin.
   *
   * @default true
   */
  enabled?: boolean;
  /**
   * Base options for SWC.
   */
  baseOptions?: SwcOptions | ((code: string, id: string) => SwcOptions);
}
