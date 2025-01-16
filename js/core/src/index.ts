let nextId = 0;

export function getNextId() {
  return ++nextId;
}

export function createEditor(doc: string, parent: HTMLElement) {
  return import(`${document.baseURI}editor-js/index.js`).then((
    mod: typeof import("../../editor/src/index"),
  ) => mod.create(doc, parent));
}
