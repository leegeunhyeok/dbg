import { describe, it, expect, vi, beforeEach, afterAll } from 'vitest';
import { _ as dbg } from './dbg';

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

  describe('without location context', () => {
    const loc = null;

    it('call with no arguments', () => {
      expect(dbg.bind(loc)()).toBeUndefined();
      expect(consoleMock).toHaveBeenCalledTimes(1);
      expect(consoleMock).toHaveBeenCalledWith('[anonymous]');
    });

    it('call with one argument', () => {
      expect(dbg.bind(loc)({ expr: '1', value: 1 })).toBe(1);
      expect(consoleMock).toHaveBeenCalledTimes(1);
      expect(consoleMock).toHaveBeenCalledWith('[anonymous] 1 =', 1);
    });

    it('call with multiple arguments', () => {
      expect(
        dbg.bind(loc)(
          { expr: '1', value: 1 },
          { expr: '{}', value: {} },
          { expr: '"foo"', value: 'foo' }
        )
      ).toEqual([1, {}, 'foo']);
      expect(consoleMock).toHaveBeenCalledTimes(3);
      expect(consoleMock).toHaveBeenCalledWith('[anonymous] 1 =', 1);
      expect(consoleMock).toHaveBeenCalledWith('[anonymous] {} =', {});
      expect(consoleMock).toHaveBeenCalledWith(`[anonymous] "foo" =`, `'foo'`);
    });
  });

  describe('with location context', () => {
    const loc = { file: 'foo.ts', line: 1, col: 2 };

    it('call with no arguments', () => {
      expect(dbg.bind(loc)()).toBeUndefined();
      expect(consoleMock).toHaveBeenCalledTimes(1);
      expect(consoleMock).toHaveBeenCalledWith('[foo.ts:1:2]');
    });

    it('call with one argument', () => {
      expect(dbg.bind(loc)({ expr: '1', value: 1 })).toBe(1);
      expect(consoleMock).toHaveBeenCalledTimes(1);
      expect(consoleMock).toHaveBeenCalledWith('[foo.ts:1:2] 1 =', 1);
    });

    it('call with multiple arguments', () => {
      expect(
        dbg.bind(loc)(
          { expr: '1', value: 1 },
          { expr: '{}', value: {} },
          { expr: '"foo"', value: 'foo' }
        )
      ).toEqual([1, {}, 'foo']);
      expect(consoleMock).toHaveBeenCalledTimes(3);
      expect(consoleMock).toHaveBeenCalledWith('[foo.ts:1:2] 1 =', 1);
      expect(consoleMock).toHaveBeenCalledWith('[foo.ts:1:2] {} =', {});
      expect(consoleMock).toHaveBeenCalledWith(`[foo.ts:1:2] "foo" =`, `'foo'`);
    });
  });
});
