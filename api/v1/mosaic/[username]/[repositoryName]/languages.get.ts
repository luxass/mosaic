export default defineLazyEventHandler(async () => {
  const ALL_LANGUAGES = await import("@luxass/github-languages/json").then((m) => m.default);

  return defineCachedEventHandler(async (event) => {
    const username = getRouterParam(event, "username");
    const repositoryName = getRouterParam(event, "repositoryName");

    if (!username || !repositoryName) {
      throw createError({
        status: 400,
        message: "missing params",
      });
    }

    const languages = await getRepositoryLanguages(username, repositoryName);

    if (!languages) {
      throw createError({
        status: 404,
        message: "repository not found",
      });
    }

    return Object.keys(languages).map((language) => ({
      name: language,
      // @ts-expect-error - doesn't matter #mutetypescript
      color: ALL_LANGUAGES[language].color ?? "#000000",
    }));
  }, {
    maxAge: 60 * 60, // 1 hour
    swr: true,
    shouldBypassCache() {
      return import.meta.dev || false;
    },
  });
});
