let publicUrl: string;

export function setPublicUrl(to: string) {
  console.log("public url", to);
  publicUrl = to;
}

export async function createEditor(doc: string, parent: HTMLElement) {
  type EditorModule = typeof import("../../editor/src/index");
  const mod: EditorModule = await import(`${publicUrl}/editor-js/index.js`);
  return mod.create(doc, parent);
}
