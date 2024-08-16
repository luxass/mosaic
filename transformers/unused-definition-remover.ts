import assert from "node:assert";
import { generated as isGenerated } from "unist-util-generated";
import { SKIP, visit } from "unist-util-visit";

import type { Root } from "mdast";
import type { Plugin } from "unified";

export const UNUSED_DEFINITION_REMOVER: Plugin<void[], Root> = function () {
  return (tree) => {
    const map = new Map<string, { used: boolean }>();

    visit(
      tree,
      [
        "imageReference",
        "linkReference",
        "footnoteReference",
        "definition",
        "footnoteDefinition",
      ],
      (node) => {
        assert(
          node.type === "imageReference" ||
            node.type === "linkReference" ||
            node.type === "footnoteReference" ||
            node.type === "definition" ||
            node.type === "footnoteDefinition",
          `unexpected node type ${node.type}`,
        );

        if (!isGenerated(node)) {
          if (
            node.type === "imageReference" ||
            node.type === "linkReference" ||
            node.type === "footnoteReference"
          ) {
            const id = node.identifier.toUpperCase();
            const info = map.get(id);

            if (info) {
              info.used = true;
            } else {
              map.set(id, { used: true });
            }
          } else {
            const id = node.identifier.toUpperCase();

            if (!map.has(id)) {
              map.set(id, { used: false });
            }
          }
        }
      },
    );

    visit(tree, ["definition", "footnoteDefinition"], (node, index, parent) => {
      assert(
        node.type === "definition" || node.type === "footnoteDefinition",
        `unexpected node type ${node.type}`,
      );

      if (!isGenerated(node)) {
        const id = node.identifier.toUpperCase();
        const info = map.get(id);

        assert(info, "new definition node magically appeared?!");
        assert(
          index !== undefined && parent !== undefined,
          "expected index and parent",
        );

        if (!info.used) {
          parent.children.splice(index, 1);
          return [SKIP, index];
        }
      }
    });
  };
};
