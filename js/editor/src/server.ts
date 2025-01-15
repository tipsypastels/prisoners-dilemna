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
  getTooltip(pos: number): { text: string; len: number } | undefined;
  getDiagnostics(): ts.Diagnostic[];
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
  const { languageService: srv } = env;

  console.log("typescript initialized");
  return {
    getAutocomplete(pos) {
      return srv.getCompletionsAtPosition(INDEX_JS, pos, {});
    },
    getTooltip(pos) {
      const info = srv.getQuickInfoAtPosition(INDEX_JS, pos);
      if (!info) {
        return;
      }

      const head = ts.displayPartsToString(info.displayParts);
      const rest = ts.displayPartsToString(info.documentation);
      const text = `${head}\n${rest}`;
      const len = info.textSpan.length;

      return { text, len };
    },
    getDiagnostics() {
      return srv.getSemanticDiagnostics(INDEX_JS);
    },
    updateFile(doc) {
      env.updateFile(INDEX_JS, doc || " ");
    },
  };
}
