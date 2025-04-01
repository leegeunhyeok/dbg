interface ExecuteContext {
  file: string;
  line: number;
  col: number;
}

interface DebugArgs {
  expr: string;
  value: any;
}

function loc(ctx: ExecuteContext | null) {
  return ctx ? `${ctx.file}:${ctx.line}:${ctx.col}` : 'anonymous';
}

function str<T>(value: T) {
  return typeof value === 'string' ? `'${value}'` : value;
}

function dbg(
  this: ExecuteContext | null,
  ...args: DebugArgs[]
): DebugArgs['value'] | DebugArgs['value'][] {
  const retValues: DebugArgs['value'][] = [];

  for (const { expr, value } of args) {
    console.log(`[${loc(this)}] ${expr} =`, str(value));
    retValues.push(value);
  }

  return retValues.length === 1 ? retValues[0] : retValues;
}

export { dbg as _ };
