export function test_hi(public_url: string | null) {
  test_import_editor(public_url).then((mod) => {
    mod.test_editor();
  });
}

async function test_import_editor(public_url: string | null) {
  const mod: typeof import("../editor/index.ts") = await import(
    `${public_url ?? ""}/editor-js/index.js`
  );
  return mod;
}
