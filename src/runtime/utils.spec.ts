import { describe, it, expect } from 'vitest';
import { loc, str, ret } from './utils';

describe('loc', () => {
  it('call with location context', () => {
    expect(
      loc({
        file: 'foo.ts',
        line: 1,
        col: 2,
      })
    ).toBe('foo.ts:1:2');
  });
});

describe('str', () => {
  it('call with string', () => {
    expect(str('foo')).toBe(`'foo'`);
  });

  it('call with non-string', () => {
    expect(str(1)).toBe(1);
    expect(str({})).toEqual({});
  });
});

describe('ret', () => {
  it('call with no arguments', () => {
    expect(ret()).toBeUndefined();
  });

  it('call with one argument', () => {
    expect(ret(1)).toBe(1);
  });

  it('call with multiple arguments', () => {
    expect(ret(1, {}, 'foo')).toEqual([1, {}, 'foo']);
  });
});
