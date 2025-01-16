let nextId = 0;
let publicUrl: string;

export function getNextId() {
  return nextId++;
}

export function setPublicUrl(to: string) {
  console.log("public url", to);
  publicUrl = to;
}

export function createEditor(doc: string, parent: HTMLElement) {
  return import(`${publicUrl}/editor-js/index.js`).then((
    mod: typeof import("../../editor/src/index"),
  ) => mod.create(doc, parent));
}
