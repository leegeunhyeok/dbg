import { ExecuteContext, Value } from './types';

export function loc(ctx: ExecuteContext | null) {
  return ctx ? `${ctx.file}:${ctx.line}:${ctx.col}` : 'anonymous';
}

export function str<T>(value: T) {
  return typeof value === 'string' ? `'${value}'` : value;
}

export function ret(...values: Value[]): void | Value | Value[] {
  if (values.length === 0) {
    return;
  }

  if (values.length === 1) {
    return values[0];
  }

  return values;
}
