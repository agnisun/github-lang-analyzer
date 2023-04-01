import { atom } from "jotai";

export const selectedDiagramAtom = atom<"pie-diagram" | "bar-diagram">(
  "pie-diagram"
);
