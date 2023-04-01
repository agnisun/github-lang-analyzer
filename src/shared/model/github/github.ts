import { atom } from "jotai";

export const githubLanguagesAtom = atom<Map<string, number>>(new Map());
export const languagesColorAtom = atom<Map<string, string>>(new Map());
export const isLoadingAtom = atom<boolean>(false);
