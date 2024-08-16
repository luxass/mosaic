import { BADGE_REMOVER } from "~~/transformers/badge-remover";
import { COMMENT_REMOVER } from "~~/transformers/remove-comments";
import { UNUSED_DEFINITION_REMOVER } from "~~/transformers/unused-definition-remover";
import { URL_REWRITER } from "~~/transformers/url-rewriter";

export default defineLazyEventHandler(async () => {
  return defineCachedEventHandler(async (event) => {
    const username = getRouterParam(event, "username");
    const repositoryName = getRouterParam(event, "repositoryName");

    if (!username || !repositoryName) {
      return new Response("missing params", {
        status: 400,
      });
    }

    const path = getRouterParam(event, "path");
    const readme = await getREADME({
      owner: username,
      repository: repositoryName,
      readmePath: path,
    });

    if (!readme || readme.type === "not_found") {
      return new Response("repository has no readme defined", {
        status: 404,
      });
    }

    if (readme.type === "error") {
      return new Response(
        "error resolving readme due to readme not being valid",
        {
          status: 500,
        },
      );
    }

    const queryParams = getQuery(event);

    const shouldTransform = getRequestHeader(event, "x-transform") === "true";

    if (shouldTransform || queryParams.transform === "" || queryParams.transform === "true") {
      const remark = await import("remark").then((m) => m.remark);

      const file = await remark()
        .use(URL_REWRITER, {
          repoUrl: `https://github.com/${username}/${repositoryName}`,
        })
        .use(BADGE_REMOVER)
        .use(UNUSED_DEFINITION_REMOVER)
        .use(COMMENT_REMOVER)
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
  });
});
