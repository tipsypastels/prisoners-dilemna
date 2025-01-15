import {
  autocompletion,
  closeBrackets,
  completeFromList,
} from "@codemirror/autocomplete";
import { indentWithTab } from "@codemirror/commands";
import { javascript } from "@codemirror/lang-javascript";
import { bracketMatching } from "@codemirror/language";
import { hoverTooltip, keymap, Tooltip } from "@codemirror/view";
import { Diagnostic, linter } from "@codemirror/lint";
import { EditorView, minimalSetup } from "codemirror";
import { createServer, Server } from "./server";

// ordered same as ts's `DiagnosticCategory` enum
const LINT_SEVERITIES: Diagnostic["severity"][] = [
  "warning",
  "error",
  "info",
  "info",
];

export function create(doc: string, parent: HTMLElement) {
  let server: Server | undefined;
  createServer(doc).then((s) => server = s);

  const view = new EditorView({
    doc,
    parent,
    dispatch: (tr) => {
      view.update([tr]);
      // TODO: This is sort of a race condition
      // if changes are made before server starts?
      if (server && tr.docChanged) {
        const doc = tr.state.doc.sliceString(0);
        server.updateFile(doc);
      }
    },
    extensions: [
      minimalSetup,
      bracketMatching(),
      closeBrackets(),
      keymap.of([indentWithTab]),
      autocompletion({
        override: [
          (ctx) => {
            if (!server) {
              return null;
            }
            const completions = server.getAutocomplete(ctx.pos);
            if (!completions) {
              return null;
            }

            return completeFromList(
              completions.entries.map((c) => ({
                type: c.kind,
                label: c.name,
                boost: 1 / Number(c.sortText),
              })),
            )(ctx);
          },
        ],
      }),
      hoverTooltip((_view, pos) => {
        if (!server) {
          return null;
        }

        const info = server.getTooltipInfo(pos);
        if (!info) {
          return null;
        }

        const end = pos + info.textSpan.length;
        const create = () => {
          const dom = document.createElement("div");
          dom.className = "cm-tooltip-cursor";
          dom.textContent = JSON.stringify(info, null, 2);
          return { dom };
        };

        return { pos, end, create } satisfies Tooltip;
      }),
      linter(() => {
        if (!server) {
          return [];
        }

        const tsDiagnostics = server.getDiagnostics();
        const diagnostics: Diagnostic[] = [];

        for (const tsD of tsDiagnostics) {
          if (tsD.start && tsD.length) {
            const from = tsD.start;
            const to = from + tsD.length;
            const source = tsD.source;
            const severity = LINT_SEVERITIES[tsD.category];
            const message = typeof tsD.messageText === "string"
              ? tsD.messageText
              : tsD.messageText.messageText;

            diagnostics.push({ from, to, source, severity, message });
          }
        }

        return diagnostics;
      }),
      javascript(),
    ],
  });
  return view;
}
