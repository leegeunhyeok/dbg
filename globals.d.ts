declare global {
  var dbg: {
    (): void;
    <T>(arg: T): T;
    <T extends any[]>(...args: [...T]): [...T];
  };
}

export {};
