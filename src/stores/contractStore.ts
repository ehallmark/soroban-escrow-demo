import { atom } from 'nanostores';

export const wallet = atom("");

let _retainees: string[] = [];
export const retainees = atom(_retainees);

let _retainors: string[] = [];
export const retainors = atom(_retainors);

export const retainorName = atom("");
export const retaineeName = atom("");

export const activePage = atom("");
