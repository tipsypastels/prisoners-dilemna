import { EditorView, minimalSetup } from "codemirror";
import { javascript } from "@codemirror/lang-javascript";
import ts from "typescript";
import * as tsvfs from "@typescript/vfs";
import lzstring from "lz-string";

export async function init(doc: string, parent: HTMLElement) {
  const { fs, program } = await init_ts(doc);
  console.log(fs.get(index_js));
  return new EditorView({
    doc,
    parent,
    extensions: [
      minimalSetup,
      javascript(),
    ],
  });
}

async function init_ts(doc: string) {
  const fs = await tsvfs.createDefaultMapFromCDN(
    compiler_options,
    ts.version,
    ts_do_cache,
    ts,
    lzstring,
  );

  fs.set(index_ts, doc);

  const system = tsvfs.createSystem(fs);
  const host = tsvfs.createVirtualCompilerHost(system, compiler_options, ts);

  const program = ts.createProgram({
    rootNames: [...fs.keys()],
    options: compiler_options,
    host: host.compilerHost,
  });

  program.emit();

  return { fs, program };
}

const index_ts = "index.ts";
const index_js = "index.js";
const ts_do_cache = true;
const compiler_options: ts.CompilerOptions = {
  strict: true,
  checkJs: true,
};

// import ts from "typescript";
// import * as tsvfs from "@typescript/vfs";
// import lzstring from "lz-string";

// export function test_editor() {
//   editor_main().then(() => {
//     console.log("editor_main DONE");
//   });
// }

// async function editor_main() {
//   const fs = await tsvfs.createDefaultMapFromCDN(
//     {},
//     ts.version,
//     true,
//     ts,
//     lzstring,
//   );
//   fs.set("index.ts", "export const main = (x: string) => x;");

//   const system = tsvfs.createSystem(fs);
//   const host = tsvfs.createVirtualCompilerHost(system, {}, ts);

//   const program = ts.createProgram({
//     rootNames: [...fs.keys()],
//     options: {},
//     host: host.compilerHost,
//   });

//   program.emit();

//   console.log(fs.get("index.js"));
//   // console.log(program.getSourceFiles().map((f) => f.fileName));
//   // const index = program.getSourceFile("index.ts");
//   // console.log(index?.text);
// }
