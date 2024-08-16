import { graphql } from "@octokit/graphql";
import type { User } from "github-schema";
import { PROFILE_CONTRIBUTIONS_QUERY } from "~/utils/graphql-queries";

export default defineLazyEventHandler(async () => {
  const runtimeConfig = useRuntimeConfig();

  return defineCachedEventHandler(async (event) => {
    const username = getRouterParam(event, "username");

    if (!username) {
      throw createError({
        status: 400,
        message: "missing params",
      });
    }

    const { user } = await graphql<{
      user: {
        contributions: User["repositoriesContributedTo"];
      };
    }>(PROFILE_CONTRIBUTIONS_QUERY, {
      headers: {
        "Authorization": `Bearer ${runtimeConfig.github.token}`,
        "Content-Type": "application/json",
      },
      login: username,
    });

    return user?.contributions?.nodes || [];
  }, {
    maxAge: 60 * 60, // 1 hour
    swr: true,
    shouldBypassCache() {
      return import.meta.dev;
    },
  });
});
