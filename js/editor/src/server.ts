import ts from "typescript";
import * as tsvfs from "@typescript/vfs";
import lzstring from "lz-string";

const INDEX_JS = "index.js";

const CACHE = true;
const COMPILER_OPTIONS: ts.CompilerOptions = {
  strict: true,
  checkJs: true,
};

export interface Server {
  getAutocomplete(pos: number): ts.CompletionInfo | undefined;
  getTooltipInfo(pos: number): ts.QuickInfo | undefined;
  updateFile(doc: string): void;
}

export async function createServer(doc: string): Promise<Server> {
  const fs = await tsvfs.createDefaultMapFromCDN(
    COMPILER_OPTIONS,
    ts.version,
    CACHE,
    ts,
    lzstring,
  );

  // Cannot be empty string.
  fs.set(INDEX_JS, doc || " ");

  const system = tsvfs.createSystem(fs);
  const env = tsvfs.createVirtualTypeScriptEnvironment(
    system,
    [INDEX_JS],
    ts,
    COMPILER_OPTIONS,
  );

  console.log("typescript initialized");
  return {
    getAutocomplete(pos) {
      return env.languageService.getCompletionsAtPosition(INDEX_JS, pos, {});
    },
    getTooltipInfo(pos) {
      return env.languageService.getQuickInfoAtPosition(INDEX_JS, pos);
    },
    updateFile(doc) {
      env.updateFile(INDEX_JS, doc || " ");
    },
  };
}
