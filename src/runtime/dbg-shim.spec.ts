import { describe, it, expect } from 'vitest';
import { _ as dbg } from './dbg-shim';

describe('dbg-shim', () => {
  it('call with no arguments', () => {
    expect(dbg()).toBeUndefined();
  });

  it('call with one argument', () => {
    expect(dbg(1)).toBe(1);
  });

  it('call with multiple arguments', () => {
    expect(dbg(1, {}, 'foo')).toEqual([1, {}, 'foo']);
  });
});
