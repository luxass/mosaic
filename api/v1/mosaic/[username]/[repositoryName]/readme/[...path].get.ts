import { ICON } from "~/transformers/icon";
import { BADGE_REMOVER } from "~~/transformers/badge-remover";
import { COMMENT_REMOVER } from "~~/transformers/remove-comments";
import { UNUSED_DEFINITION_REMOVER } from "~~/transformers/unused-definition-remover";
import { URL_REWRITER } from "~~/transformers/url-rewriter";

export default defineLazyEventHandler(async () => {
  const remark = await import("remark").then((m) => m.remark);
  return defineCachedEventHandler(async (event) => {
    const username = getRouterParam(event, "username");
    const repositoryName = getRouterParam(event, "repositoryName");

    if (!username || !repositoryName) {
      throw createError({
        status: 400,
        message: "missing username or repository name",
      });
    }

    const path = getRouterParam(event, "path");
    const readme = await getREADME({
      owner: username,
      repository: repositoryName,
      readmePath: path,
    });

    if (!readme || readme.type === "not_found") {
      throw createError({
        status: 404,
        message: "repository has no readme defined",
      });
    }

    if (readme.type === "error") {
      throw createError({
        status: 500,
        message: "error resolving readme",
        data: readme.details,
      });
    }

    const queryParams = getQuery(event);

    const shouldTransform = getRequestHeader(event, "x-transform") === "true";

    if (shouldTransform || queryParams.transform === "" || queryParams.transform === "true") {
      const projectName = getRequestHeader(event, "x-transform-name");

      if (!projectName?.trim()) {
        throw createError({
          status: 400,
          message: "missing project name",
        });
      }

      const ICONS: Map<string, string> = new Map();

      const file = await remark()
        .use(URL_REWRITER, {
          repoUrl: `https://github.com/${username}/${repositoryName}`,
        })
        .use(BADGE_REMOVER)
        .use(UNUSED_DEFINITION_REMOVER)
        .use(COMMENT_REMOVER)
        .use(ICON, {
          icons: ICONS,
          name: projectName,
        })
        .process(readme.content || "No README was found.");

      const transformedContent = file.toString();

      return {
        lastModified: new Date().toISOString(),
        content: transformedContent,
        path: readme.path,
      };
    }

    return {
      lastModified: new Date().toISOString(),
      content: readme.content,
      path: readme.path,
    };
  }, {
    maxAge: 60 * 60, // 1 hour
    swr: true,
    shouldBypassCache() {
      return import.meta.dev;
    },
    varies: [
      "x-transform",
      "x-transform-name",
    ],
  });
});
