export default defineLazyEventHandler(async () => {
  return defineCachedEventHandler(async (event) => {
    const username = getRouterParam(event, "username");
    const repositoryName = getRouterParam(event, "repositoryName");

    if (!username || !repositoryName) {
      throw createError({
        status: 400,
        message: "missing params",
      });
    }

    const resolvedMosaicConfig = await resolveMosaicConfig(username, repositoryName);

    if (!resolvedMosaicConfig || resolvedMosaicConfig.type === "not_found") {
      throw createError({
        status: 404,
        message: "repository has no config defined",
      });
    }

    if (resolvedMosaicConfig.type === "error") {
      throw createError({
        status: 400,
        message: "error resolving config due to config not being valid",
        data: resolvedMosaicConfig.details,
      });
    }

    return {
      lastModified: new Date().toISOString(),
      content: resolvedMosaicConfig.content,
      external: resolvedMosaicConfig.external,
      path: resolvedMosaicConfig.path,
    };
  }, {
    maxAge: 60 * 60, // 1 hour
    swr: true,
    shouldBypassCache() {
      return import.meta.dev || false;
    },
  });
});
