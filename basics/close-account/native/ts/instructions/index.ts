export * from "./close.ts";
export * from "./create.ts";

export const MyInstruction = {
  CreateUser: 0,
  CloseUser: 1,
} as const;
