let public_url: string;

export function set_public_url(to: string) {
  console.log("public url", to);
  public_url = to;
}

export function test_hi() {
  test_import_editor().then((mod) => {
    mod.test_editor();
  });
}

async function test_import_editor() {
  const mod: typeof import("../editor/index.ts") = await import(
    `${public_url}/editor-js/index.js`
  );
  return mod;
}
