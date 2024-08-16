import { graphql } from "@octokit/graphql";
import type { Repository, User } from "github-schema";

export default defineLazyEventHandler(async () => {
  const runtimeConfig = useRuntimeConfig();

  console.log("fetching projects");

  return defineCachedEventHandler(async (event) => {
    console.time("total-time");

    console.time("fetch-profile");
    const { viewer } = await graphql<{
      viewer: Omit<User, "repositoriesContributedTo"> & {
        contributions: User["repositoriesContributedTo"];
      };
    }>(PROFILE_QUERY, {
      headers: {
        "Authorization": `Bearer ${runtimeConfig.github.token}`,
        "Content-Type": "application/json",
      },
    });

    console.timeEnd("fetch-profile");

    if (!viewer.repositories.nodes?.length) {
      return [];
    }

    const projects: {
      name: string;
      nameWithOwner: string;
      mosaicUrl: string;
    }[] = [];

    console.time("fetch-ignore");
    const ignoreFile = await fetch(
      "https://raw.githubusercontent.com/luxass/luxass/main/.github/mosaic/.mosaicignore",
    ).then((res) => res.text());
    const ignore = ignoreFile
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line && !line.startsWith("#"));

    console.timeEnd("fetch-ignore");

    const repositories = viewer.repositories.nodes.filter(
      (repo): repo is NonNullable<Repository> => {
        return (
          !!repo
          && !repo.isFork
          && !repo.isPrivate
          && !repo.isArchived
          && !ignore.includes(repo.nameWithOwner)
          && !ignore.includes(repo.nameWithOwner.split("/")[1])
        );
      },
    );

    // for await (const file of getExternalRepositories()) {
    //   if (file.endsWith("README.md") || file.endsWith(".mosaicignore")) continue;

    //   const [owner, name] = file.replace(".github/mosaic/", "").split("/");

    //   const { repository } = await graphql<{
    //     repository: Repository;
    //   }>(REPOSITORY_QUERY, {
    //     owner,
    //     name: name.replace(".toml", ""),
    //     headers: {
    //       "Authorization": `Bearer ${runtimeConfig.github.token}`,
    //       "Content-Type": "application/json",
    //     },
    //   });

    //   repositories.push(repository);
    // }

    console.timeEnd("total-time");
    return projects;
  }, {
    maxAge: 60 * 60, // 1 hour
    swr: true,
    shouldBypassCache() {
      return import.meta.dev;
    },
  });
});
