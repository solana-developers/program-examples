export * from "./close";
export * from "./create";

export const MyInstruction = {
  CreateUser: 0,
  CloseUser: 1,
} as const;
