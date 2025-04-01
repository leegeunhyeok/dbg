declare global {
  var dbg: {
    <T>(arg: T): T;
    <T extends any[]>(...args: [...T]): [...T];
  };
}

export {};
