import { visit } from "unist-util-visit";
import type { Root } from "mdast";
import type { Plugin } from "unified";

interface Options {
  repoUrl: string;
}

export const URL_REWRITER: Plugin<Options[], Root> = (options: Options) => {
  return (tree) => {
    visit(tree, "link", (node) => {
      if (!node?.url) {
        throw new Error("missing url");
      }

      if (node.url.startsWith("http")) {
        return;
      }

      node.url = new URL(node.url, `${options.repoUrl}/blob/main/`).toString();
    });
  };
};
