interface ExecuteContext {
  file: string;
  line: number;
  col: number;
}

interface DebugArgs {
  expr: string;
  value: Value;
}

type Value = any;

function loc(ctx: ExecuteContext | null) {
  return ctx ? `${ctx.file}:${ctx.line}:${ctx.col}` : 'anonymous';
}

function str<T>(value: T) {
  return typeof value === 'string' ? `'${value}'` : value;
}

function ret(...values: Value[]) {
  return values.length === 1 ? values[0] : values;
}

function dbg(
  this: ExecuteContext | null,
  ...args: DebugArgs[]
): Value | Value[] {
  const retValues: Value[] = [];

  for (const { expr, value } of args) {
    console.log(`[${loc(this)}] ${expr} =`, str(value));
    retValues.push(value);
  }

  return ret(...retValues);
}

dbg.shim = ret;

export { dbg as _ };
