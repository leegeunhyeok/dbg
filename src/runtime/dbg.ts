import type { ExecuteContext, Value } from './types';
import { loc, str, ret } from './utils';

interface DebugArgs {
  expr: string;
  value: Value;
}

function dbg(
  this: ExecuteContext | null,
  ...args: DebugArgs[]
): Value | Value[] {
  const retValues: Value[] = [];
  const locLabel = `[${loc(this)}]`;

  if (args.length === 0) {
    console.log(locLabel);
  } else {
    for (const { expr, value } of args) {
      console.log(`${locLabel} ${expr} =`, str(value));
      retValues.push(value);
    }
  }

  return ret(...retValues);
}

export { dbg as _ };
