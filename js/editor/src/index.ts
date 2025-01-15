import { closeBrackets } from "@codemirror/autocomplete";
import { indentWithTab } from "@codemirror/commands";
import { javascript } from "@codemirror/lang-javascript";
import { bracketMatching } from "@codemirror/language";
import { hoverTooltip, keymap, Tooltip } from "@codemirror/view";
import { EditorView, minimalSetup } from "codemirror";
import { createServer, Server } from "./server";

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
      javascript(),
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
    ],
  });
  return view;
}
