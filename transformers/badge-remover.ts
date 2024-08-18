import { SKIP, visit } from "unist-util-visit";
import type { ImageReference, Node, Root } from "mdast";
import type { Plugin } from "unified";
import { type GetDefinition, definitions } from "mdast-util-definitions";

const BADGE_SRC = [
  "https://img.shields.io",
  "https://flat.badgen.net/",
  "https://jsr.io/badges/",
];

function isBadge(url: string): boolean {
  for (const src of BADGE_SRC) {
    if (url.startsWith(src)) {
      return true;
    }
  }

  return false;
}

function badgeImage(node: Node, define: GetDefinition) {
  if (node.type === "imageReference") {
    const def = define((node as ImageReference).identifier);
    return def ? isBadge(def.url) : false;
  }

  if (!("url" in node) || typeof node.url !== "string") return false;

  return node.type === "image" ? isBadge(node.url) : false;
}

export const BADGE_REMOVER: Plugin<void[], Root> = () => {
  return (tree) => {
    const define = definitions(tree);

    visit(tree, (node, index, parent) => {
      let remove = false;

      if (node.type === "link" || node.type === "linkReference") {
        const children = node.children;
        let offset = -1;

        while (++offset < children.length) {
          const child = children[offset];

          if (badgeImage(child as any, define)) {
            remove = true;
            break;
          }
        }
      } else if (badgeImage(node, define)) {
        remove = true;
      }

      if (remove === true && parent && typeof index === "number") {
        parent.children.splice(index, 1);

        // remove the definition node

        if (index === parent.children.length) {
          let tail = parent.children[index - 1];

          // If the remaining tail is a text.
          while (tail && tail.type === "text") {
            index--;

            // Remove trailing tabs and spaces.
            tail.value = tail.value.replace(/[ \t]+$/, "");

            // Remove the whole if it was whitespace only.
            if (!tail.value) {
              parent.children.splice(index, 1);
            }

            tail = parent.children[index - 1];
          }
        }

        return [SKIP, index];
      }
    });
  };
};
