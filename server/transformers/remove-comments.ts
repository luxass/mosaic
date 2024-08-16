import { visit } from "unist-util-visit";
import type { Root } from "mdast";
import type { Plugin } from "unified";

export const COMMENT_REMOVER: Plugin<void[], Root> = () => {
  return (tree) => {
    visit(tree, "html", (node) => {
      if (node.value.startsWith("<!--")) {
        node.value = "";
      }
    });
  };
};
