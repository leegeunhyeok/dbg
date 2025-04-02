import type { ParserConfig } from '@swc/core';

export function mergeArray(objValue: any[], srcValue: any[]) {
  if (Array.isArray(objValue)) {
    return objValue.concat(srcValue);
  }
}

export function getBaseSwcParserConfig(id: string): ParserConfig | undefined {
  let jsx = false;
  let syntax: ParserConfig['syntax'] | null = null;

  switch (true) {
    // JavaScript
    case id.endsWith('.mdx'):
    case id.endsWith('.jsx'):
      jsx = true;
    case id.endsWith('.js'):
      syntax = 'ecmascript';
      break;

    // TypeScript
    case id.endsWith('.tsx'):
      jsx = true;
    case id.endsWith('.ts'):
      syntax = 'typescript';
      break;
  }

  switch (syntax) {
    case 'ecmascript':
      return { syntax, jsx };
    case 'typescript':
      return { syntax, tsx: jsx };
    default:
      return undefined;
  }
}
