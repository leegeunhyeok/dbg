import { describe, it, expect, vi, beforeEach, afterAll } from 'vitest';
import { _ as dbg } from './dbg-shim';

describe('dbg-shim', () => {
  const consoleMock = vi
    .spyOn(console, 'log')
    .mockImplementation(() => undefined);

  beforeEach(() => {
    consoleMock.mockClear();
  });

  afterAll(() => {
    consoleMock.mockReset();
  });

  it('call with no arguments', () => {
    expect(dbg()).toBeUndefined();
    expect(consoleMock).toHaveBeenCalledTimes(0);
  });

  it('call with one argument', () => {
    expect(dbg(1)).toBe(1);
    expect(consoleMock).toHaveBeenCalledTimes(0);
  });

  it('call with multiple arguments', () => {
    expect(dbg(1, {}, 'foo')).toEqual([1, {}, 'foo']);
    expect(consoleMock).toHaveBeenCalledTimes(0);
  });
});
