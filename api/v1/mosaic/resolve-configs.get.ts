import { z } from "zod";

const GITHUB_REPOSITORIES = z.array(z.string()).max(10);

export default defineLazyEventHandler(async () => {
  return defineEventHandler(async (event) => {
    const repositoriesRaw = getRequestHeader(event, "x-mosaic-repositories");

    if (!repositoriesRaw?.trim()) {
      throw createError({
        status: 400,
        message: "missing x-mosaic-repositories header",
      });
    }

    const splittedRepositories = repositoriesRaw.trim().split(",");

    if (splittedRepositories.length > 10) {
      throw createError({
        status: 400,
        message: "too many repositories in single request",
      });
    }

    const result = GITHUB_REPOSITORIES.safeParse(splittedRepositories);
    if (!result.success) {
      console.error("invalid repositories", result.error);
      throw createError({
        status: 400,
        message: "invalid repositories",
        data: result.error,
      });
    }

    const repositories = result.data;

    const configs = await Promise.all(repositories.map(async (repository) => {
      const [username, repositoryName] = repository.split("/");
      return {
        repository,
        resolvedMosaicConfig: await resolveMosaicConfig(username, repositoryName),
      };
    }));

    return configs.map(({ repository, resolvedMosaicConfig }) => {
      if (!resolvedMosaicConfig || resolvedMosaicConfig.type === "not_found") {
        return {
          repository,
          type: "not_found",
        };
      }

      if (resolvedMosaicConfig.type === "error") {
        return {
          repository,
          type: "error",
          details: resolvedMosaicConfig.details,
        };
      }

      return {
        repository,
        type: "success",
        content: resolvedMosaicConfig.content,
        external: resolvedMosaicConfig.external,
      };
    });
  });
});
