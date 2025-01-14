let public_url: string;

export function set_public_url(to: string) {
  console.log("public url", to);
  public_url = to;
}

export async function editor_init(doc: string, parent: HTMLElement) {
  type EditorModule = typeof import("../../editor/src/index");
  const mod: EditorModule = await import(`${public_url}/editor-js/index.js`);
  return await mod.init(doc, parent);
}
