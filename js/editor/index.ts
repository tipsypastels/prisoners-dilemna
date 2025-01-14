import ts from "typescript";
import * as tsvfs from "@typescript/vfs";
import lzstring from "lz-string";

export function test_editor() {
  editor_main().then(() => {
    console.log("editor_main DONE");
  });
}

async function editor_main() {
  const fs = await tsvfs.createDefaultMapFromCDN(
    {},
    ts.version,
    true,
    ts,
    lzstring,
  );
  fs.set("index.ts", "export const main = (x: string) => x;");

  const system = tsvfs.createSystem(fs);
  const host = tsvfs.createVirtualCompilerHost(system, {}, ts);

  const program = ts.createProgram({
    rootNames: [...fs.keys()],
    options: {},
    host: host.compilerHost,
  });

  program.emit();

  console.log(fs.get("index.js"));
  // console.log(program.getSourceFiles().map((f) => f.fileName));
  // const index = program.getSourceFile("index.ts");
  // console.log(index?.text);
}
