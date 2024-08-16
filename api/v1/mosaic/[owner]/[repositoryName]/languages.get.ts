export default defineLazyEventHandler(async () => {
  const ALL_LANGUAGES = await import("@luxass/github-languages/json").then((m) => m.default);

  return defineCachedEventHandler(async (event) => {
    const owner = getRouterParam(event, "owner");
    const repositoryName = getRouterParam(event, "repositoryName");

    if (!owner || !repositoryName) {
      throw createError({
        status: 400,
        message: "missing params",
      });
    }

    const languages = await getRepositoryLanguages(owner, repositoryName);

    if (!languages) {
      throw createError({
        status: 404,
        message: "repository not found",
      });
    }

    return Object.keys(languages).map((language) => ({
      name: language,
      color: ALL_LANGUAGES[language]?.color ?? "#000000",
    }));
  }, {
    maxAge: 60 * 60, // 1 hour
    swr: true,
    shouldBypassCache() {
      return import.meta.dev;
    },
  });
});
